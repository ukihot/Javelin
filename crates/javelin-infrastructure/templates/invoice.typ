// --- 定数・カラー定義 ---
#let phi = 1.618
#let primary-navy = rgb("#111827")
#let secondary-gray = rgb("#4b5563")
#let bg-gray = rgb("#f9fafb")
#let alert-red = rgb("#b91c1c")
#let success-green = rgb("#10b981")
#let proforma-blue = rgb("#0ea5e9")

// --- ヘルパー関数 ---
#let format-currency(n) = {
  let s = str(calc.round(n))
  let result = ""
  let count = 0
  for i in range(s.len() - 1, -1, step: -1) {
    if count > 0 and calc.rem(count, 3) == 0 { result = "," + result }
    result = s.at(i) + result
    count += 1
  }
  "¥" + result
}

#let mono-text(it) = text(font: "Courier New", weight: "medium", it)

// ステータスバッジの描画
#let status-badge(label, fill-color) = {
  box(
    rect(
      fill: fill-color,
      radius: 2pt,
      inset: (x: 5pt, y: 3.5pt),
      text(fill: white, size: 0.7em, weight: "bold", tracking: 0.08em)[#label],
    ),
  )
}

// --- メインテンプレート ---
#let invoice(
  receiver: "",
  invoice-no: "",
  date: "",
  due-date: "",
  issue-type: "ORIGINAL", // ORIGINAL, DUPLICATE, REVISED
  document-phase: "COMMERCIAL", // COMMERCIAL, PROFORMA
  settlement: "UNPAID", // UNPAID, PAID, OVERDUE
  registration-no: "",
  issuer: (name: "", department: "", address: "", email: "", tel: ""),
  bank-info: (bank: "", branch: "", type: "", number: "", name: ""),
  items: (),
  doc,
) = {
  let rates = items.map(it => it.tax-rate).dedup().sorted()
  let summary = rates.map(rate => {
    let target-items = items.filter(it => it.tax-rate == rate)
    let subtotal = target-items.map(it => it.price * it.qty).sum()
    let tax = calc.floor(subtotal * rate / 100)
    (rate: rate, subtotal: subtotal, tax: tax)
  })
  let grand-total = summary.map(s => s.subtotal + s.tax).sum()

  let is-proforma = (document-phase == "PROFORMA")
  let main-title = if is-proforma { "PROFORMA INVOICE" } else { "COMMERCIAL INVOICE" }
  let title-color = if is-proforma { proforma-blue } else { primary-navy }

  // ページ設定
  set page(
    paper: "a4",
    margin: (x: 1.5cm, y: 1.5cm * (2 - phi)),
    background: if settlement == "PAID" {
      rotate(-30deg, text(15em, fill: success-green.lighten(92%))[PAID])
    },
  )

  set text(font: ("ShipporiMincho"), size: 9pt, fill: primary-navy, lang: "jp")

  // 1. ヘッダー
  grid(
    columns: (phi * 1fr, 1fr),
    [
      #text(size: 2.2em, weight: 300, tracking: 0.1em, fill: title-color)[#main-title] \
      #v(0.8em)
      #text(size: 1.1em, weight: "bold")[#receiver 御中]
      #v(0.2em)
      #text(size: 0.9em, fill: secondary-gray)[下記の通り、御請求申し上げます。]
    ],
    [
      #set align(right)
      // バッジを発行者情報の真上に、右端揃えで配置
      #stack(
        dir: ltr,
        spacing: 0.4em,
        if issue-type != "ORIGINAL" { status-badge(issue-type, secondary-gray) },
        if settlement == "PAID" { status-badge("PAID", success-green) } else if settlement == "OVERDUE" {
          status-badge("OVERDUE", alert-red)
        },
      )
      #v(0.6em)
      #text(weight: "bold", size: 1.2em)[#issuer.name] \
      #if issuer.department != "" [#text(size: 0.9em)[#issuer.department]] \
      #v(0.3em)
      #set text(fill: secondary-gray, size: 0.85em)
      #issuer.address \
      #issuer.tel \
      #if registration-no != "" [登録番号: #registration-no]
    ],
  )

  v(4em)

  // 2. メタデータ
  block(
    fill: bg-gray,
    inset: 1.2em,
    radius: 4pt,
    stroke: (left: 3pt + title-color),
    grid(
      columns: (1fr, 1fr, 1fr, phi * 1fr),
      stack(spacing: 0.6em, text(fill: secondary-gray, size: 0.8em)[Invoice No.], text(weight: "bold")[#invoice-no]),
      stack(spacing: 0.6em, text(fill: secondary-gray, size: 0.8em)[Date of Issue], text(weight: "bold")[#date]),
      stack(spacing: 0.6em, text(fill: secondary-gray, size: 0.8em)[Due Date], text(weight: "bold", fill: if settlement
        == "OVERDUE" { alert-red } else { primary-navy })[#due-date]),
      [
        #set align(right)
        #text(fill: secondary-gray, size: 0.8em)[Total Amount (Tax Incl.)] \
        #v(0.2em)
        #text(size: 1.8em, weight: "bold", fill: title-color)[#mono-text(format-currency(grand-total))]
      ],
    ),
  )

  v(2.5em)

  // 3. 明細
  table(
    columns: (1fr, 80pt, 40pt, 90pt),
    inset: (y: 12pt, x: 5pt),
    stroke: (y: 0.5pt + rgb("#e5e7eb"), x: none),
    fill: (x, y) => if y == 0 { bg-gray } else { none },
    table.header(
      [#text(fill: secondary-gray, size: 0.8em, weight: "bold")[DESCRIPTION]],
      [#set align(right); #text(fill: secondary-gray, size: 0.8em, weight: "bold")[UNIT PRICE]],
      [#set align(right); #text(fill: secondary-gray, size: 0.8em, weight: "bold")[QTY]],
      [#set align(right); #text(fill: secondary-gray, size: 0.8em, weight: "bold")[AMOUNT]],
    ),
    ..items
      .map(it => (
        [#it.name #text(size: 0.75em, fill: secondary-gray.lighten(20%))[ \ (#it.tax-rate% VAT)]],
        [#set align(right); #mono-text(format-currency(it.price))],
        [#set align(right); #mono-text(str(it.qty))],
        [#set align(right); #mono-text(format-currency(it.price * it.qty))],
      ))
      .flatten(),
  )

  // 4. 計算サマリー
  grid(
    columns: (1.8fr, 1fr),
    [],
    [
      #set align(right)
      #v(0.5em)
      #table(
        columns: (1fr, 1fr),
        stroke: none,
        inset: (y: 5pt, x: 0pt),
        ..summary
          .map(s => (
            [#text(fill: secondary-gray)[Taxable (#s.rate%):]],
            [#mono-text(format-currency(s.subtotal))],
            [#text(fill: secondary-gray)[Tax Amount:]],
            [#mono-text(format-currency(s.tax))],
          ))
          .flatten(),
        table.hline(stroke: 1.5pt + title-color),
        [#v(0.5em) #text(weight: "bold")[GRAND TOTAL]],
        [#v(0.5em) #text(size: 1.3em, weight: "bold", fill: title-color)[#mono-text(format-currency(grand-total))]],
      )
    ],
  )

  v(1fr)

  // 5. お振込先
  block(
    width: 100%,
    stroke: 0.5pt + rgb("#e5e7eb"),
    inset: 1.5em,
    radius: 4pt,
    [
      #text(weight: "bold", size: 0.9em, spacing: 0.2em)[PAYMENT INFORMATION]
      #v(1em)
      #grid(
        columns: (auto, 1fr, auto, 1fr),
        column-gutter: 2em,
        row-gutter: 0.8em,
        text(fill: secondary-gray)[Bank / Branch],
        [#bank-info.bank #bank-info.branch Branch],
        text(fill: secondary-gray)[Account Type],
        [#bank-info.type],

        text(fill: secondary-gray)[Account No.],
        [#mono-text(bank-info.number)],
        text(fill: secondary-gray)[Holder],
        [#bank-info.name],
      )
    ],
  )

  v(1.5em)
  text(size: 0.75em, fill: secondary-gray)[
    NOTES: Please note that the bank transfer fee will be borne by the customer.
  ]
  doc
}
