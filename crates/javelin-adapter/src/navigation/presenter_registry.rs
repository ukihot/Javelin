// PresenterRegistry - Global registry mapping page instances to presenters
// Enables controllers to find the correct presenter for each page instance

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use uuid::Uuid;

use crate::presenter::{
    AccountMasterPresenter, ApplicationSettingsPresenter, BatchHistoryPresenter,
    CompanyMasterPresenter, JournalEntryPresenter, SearchPresenter,
    SubsidiaryAccountMasterPresenter,
};

/// Global registry mapping page instances to presenters
///
/// When a page creates its own channels, it also creates presenters
/// and registers them here. Controllers use this registry to find
/// the correct presenter for the current page instance.
///
/// # Thread Safety
///
/// Uses RwLock for thread-safe concurrent access. Multiple readers
/// can access the registry simultaneously, but writers have exclusive access.
///
/// # Example
///
/// ```rust,ignore
/// // In SearchPageState::new()
/// let id = Uuid::new_v4();
/// let presenter = Arc::new(SearchPresenter::new(...));
/// registry.register_search_presenter(id, presenter);
///
/// // In SearchController
/// let presenter = registry.get_search_presenter(page_id)?;
/// interactor.search(criteria, presenter).await
///
/// // In SearchPageState::drop()
/// registry.unregister_search_presenter(id);
/// ```
#[derive(Clone)]
pub struct PresenterRegistry {
    search_presenters: Arc<RwLock<HashMap<Uuid, Arc<SearchPresenter>>>>,
    journal_entry_presenters: Arc<RwLock<HashMap<Uuid, Arc<JournalEntryPresenter>>>>,
    account_master_presenters: Arc<RwLock<HashMap<Uuid, Arc<AccountMasterPresenter>>>>,
    company_master_presenters: Arc<RwLock<HashMap<Uuid, Arc<CompanyMasterPresenter>>>>,
    application_settings_presenters: Arc<RwLock<HashMap<Uuid, Arc<ApplicationSettingsPresenter>>>>,
    subsidiary_account_master_presenters:
        Arc<RwLock<HashMap<Uuid, Arc<SubsidiaryAccountMasterPresenter>>>>,
    batch_history_presenters: Arc<RwLock<HashMap<Uuid, Arc<BatchHistoryPresenter>>>>,
}

impl PresenterRegistry {
    /// Create a new empty presenter registry
    pub fn new() -> Self {
        Self {
            search_presenters: Arc::new(RwLock::new(HashMap::new())),
            journal_entry_presenters: Arc::new(RwLock::new(HashMap::new())),
            account_master_presenters: Arc::new(RwLock::new(HashMap::new())),
            company_master_presenters: Arc::new(RwLock::new(HashMap::new())),
            application_settings_presenters: Arc::new(RwLock::new(HashMap::new())),
            subsidiary_account_master_presenters: Arc::new(RwLock::new(HashMap::new())),
            batch_history_presenters: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Search Presenter methods

    /// Register a search presenter for a page instance
    pub fn register_search_presenter(&self, id: Uuid, presenter: Arc<SearchPresenter>) {
        self.search_presenters.write().unwrap().insert(id, presenter);
    }

    /// Get a search presenter by page instance ID
    pub fn get_search_presenter(&self, id: Uuid) -> Option<Arc<SearchPresenter>> {
        self.search_presenters.read().unwrap().get(&id).cloned()
    }

    /// Unregister a search presenter
    pub fn unregister_search_presenter(&self, id: Uuid) {
        self.search_presenters.write().unwrap().remove(&id);
    }

    // Journal Entry Presenter methods

    /// Register a journal entry presenter for a page instance
    pub fn register_journal_entry_presenter(
        &self,
        id: Uuid,
        presenter: Arc<JournalEntryPresenter>,
    ) {
        self.journal_entry_presenters.write().unwrap().insert(id, presenter);
    }

    /// Get a journal entry presenter by page instance ID
    pub fn get_journal_entry_presenter(&self, id: Uuid) -> Option<Arc<JournalEntryPresenter>> {
        self.journal_entry_presenters.read().unwrap().get(&id).cloned()
    }

    /// Unregister a journal entry presenter
    pub fn unregister_journal_entry_presenter(&self, id: Uuid) {
        self.journal_entry_presenters.write().unwrap().remove(&id);
    }

    // Account Master Presenter methods

    /// Register an account master presenter for a page instance
    pub fn register_account_master_presenter(
        &self,
        id: Uuid,
        presenter: Arc<AccountMasterPresenter>,
    ) {
        self.account_master_presenters.write().unwrap().insert(id, presenter);
    }

    /// Get an account master presenter by page instance ID
    pub fn get_account_master_presenter(&self, id: Uuid) -> Option<Arc<AccountMasterPresenter>> {
        self.account_master_presenters.read().unwrap().get(&id).cloned()
    }

    /// Unregister an account master presenter
    pub fn unregister_account_master_presenter(&self, id: Uuid) {
        self.account_master_presenters.write().unwrap().remove(&id);
    }

    // Company Master Presenter methods

    /// Register a company master presenter for a page instance
    pub fn register_company_master_presenter(
        &self,
        id: Uuid,
        presenter: Arc<CompanyMasterPresenter>,
    ) {
        self.company_master_presenters.write().unwrap().insert(id, presenter);
    }

    /// Get a company master presenter by page instance ID
    pub fn get_company_master_presenter(&self, id: Uuid) -> Option<Arc<CompanyMasterPresenter>> {
        self.company_master_presenters.read().unwrap().get(&id).cloned()
    }

    /// Unregister a company master presenter
    pub fn unregister_company_master_presenter(&self, id: Uuid) {
        self.company_master_presenters.write().unwrap().remove(&id);
    }

    // Application Settings Presenter methods

    /// Register an application settings presenter for a page instance
    pub fn register_application_settings_presenter(
        &self,
        id: Uuid,
        presenter: Arc<ApplicationSettingsPresenter>,
    ) {
        self.application_settings_presenters.write().unwrap().insert(id, presenter);
    }

    /// Get an application settings presenter by page instance ID
    pub fn get_application_settings_presenter(
        &self,
        id: Uuid,
    ) -> Option<Arc<ApplicationSettingsPresenter>> {
        self.application_settings_presenters.read().unwrap().get(&id).cloned()
    }

    /// Unregister an application settings presenter
    pub fn unregister_application_settings_presenter(&self, id: Uuid) {
        self.application_settings_presenters.write().unwrap().remove(&id);
    }

    // Subsidiary Account Master Presenter methods

    /// Register a subsidiary account master presenter for a page instance
    pub fn register_subsidiary_account_master_presenter(
        &self,
        id: Uuid,
        presenter: Arc<SubsidiaryAccountMasterPresenter>,
    ) {
        self.subsidiary_account_master_presenters.write().unwrap().insert(id, presenter);
    }

    /// Get a subsidiary account master presenter by page instance ID
    pub fn get_subsidiary_account_master_presenter(
        &self,
        id: Uuid,
    ) -> Option<Arc<SubsidiaryAccountMasterPresenter>> {
        self.subsidiary_account_master_presenters.read().unwrap().get(&id).cloned()
    }

    /// Unregister a subsidiary account master presenter
    pub fn unregister_subsidiary_account_master_presenter(&self, id: Uuid) {
        self.subsidiary_account_master_presenters.write().unwrap().remove(&id);
    }

    // Batch History Presenter methods

    /// Register a batch history presenter for a page instance
    pub fn register_batch_history_presenter(
        &self,
        id: Uuid,
        presenter: Arc<BatchHistoryPresenter>,
    ) {
        self.batch_history_presenters.write().unwrap().insert(id, presenter);
    }

    /// Get a batch history presenter by page instance ID
    pub fn get_batch_history_presenter(&self, id: Uuid) -> Option<Arc<BatchHistoryPresenter>> {
        self.batch_history_presenters.read().unwrap().get(&id).cloned()
    }

    /// Unregister a batch history presenter
    pub fn unregister_batch_history_presenter(&self, id: Uuid) {
        self.batch_history_presenters.write().unwrap().remove(&id);
    }

    // Utility methods

    /// Get the total number of registered presenters across all types
    pub fn total_count(&self) -> usize {
        self.search_presenters.read().unwrap().len()
            + self.journal_entry_presenters.read().unwrap().len()
            + self.account_master_presenters.read().unwrap().len()
            + self.company_master_presenters.read().unwrap().len()
            + self.application_settings_presenters.read().unwrap().len()
            + self.subsidiary_account_master_presenters.read().unwrap().len()
            + self.batch_history_presenters.read().unwrap().len()
    }

    /// Clear all registered presenters (useful for testing)
    pub fn clear_all(&self) {
        self.search_presenters.write().unwrap().clear();
        self.journal_entry_presenters.write().unwrap().clear();
        self.account_master_presenters.write().unwrap().clear();
        self.company_master_presenters.write().unwrap().clear();
        self.application_settings_presenters.write().unwrap().clear();
        self.subsidiary_account_master_presenters.write().unwrap().clear();
        self.batch_history_presenters.write().unwrap().clear();
    }
}

impl Default for PresenterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_search_presenter_registration_and_retrieval() {
        let registry = PresenterRegistry::new();
        let id = Uuid::new_v4();

        // Create dummy channels for presenter
        let (result_tx, _) = tokio::sync::mpsc::channel(100);
        let (error_tx, _) = tokio::sync::mpsc::channel(100);
        let (progress_tx, _) = tokio::sync::mpsc::channel(100);
        let (execution_time_tx, _) = tokio::sync::mpsc::channel(100);

        let presenter =
            Arc::new(SearchPresenter::new(result_tx, error_tx, progress_tx, execution_time_tx));

        // Register presenter
        registry.register_search_presenter(id, Arc::clone(&presenter));

        // Retrieve presenter
        let retrieved = registry.get_search_presenter(id);
        assert!(retrieved.is_some());
        assert_eq!(registry.total_count(), 1);
    }

    #[test]
    fn test_search_presenter_unregistration() {
        let registry = PresenterRegistry::new();
        let id = Uuid::new_v4();

        let (result_tx, _) = tokio::sync::mpsc::channel(100);
        let (error_tx, _) = tokio::sync::mpsc::channel(100);
        let (progress_tx, _) = tokio::sync::mpsc::channel(100);
        let (execution_time_tx, _) = tokio::sync::mpsc::channel(100);

        let presenter =
            Arc::new(SearchPresenter::new(result_tx, error_tx, progress_tx, execution_time_tx));

        registry.register_search_presenter(id, presenter);
        assert_eq!(registry.total_count(), 1);

        // Unregister presenter
        registry.unregister_search_presenter(id);
        assert_eq!(registry.total_count(), 0);
        assert!(registry.get_search_presenter(id).is_none());
    }

    #[test]
    fn test_journal_entry_presenter_registration() {
        let registry = PresenterRegistry::new();
        let id = Uuid::new_v4();

        let (list_tx, _) = tokio::sync::mpsc::unbounded_channel();
        let (detail_tx, _) = tokio::sync::mpsc::unbounded_channel();
        let (result_tx, _) = tokio::sync::mpsc::unbounded_channel();
        let (progress_tx, _) = tokio::sync::mpsc::unbounded_channel();

        let presenter =
            Arc::new(JournalEntryPresenter::new(list_tx, detail_tx, result_tx, progress_tx));

        registry.register_journal_entry_presenter(id, Arc::clone(&presenter));

        let retrieved = registry.get_journal_entry_presenter(id);
        assert!(retrieved.is_some());
        assert_eq!(registry.total_count(), 1);
    }

    #[test]
    fn test_account_master_presenter_registration() {
        let registry = PresenterRegistry::new();
        let id = Uuid::new_v4();

        let (tx, _) = tokio::sync::mpsc::unbounded_channel();
        let presenter = Arc::new(AccountMasterPresenter::new(tx));

        registry.register_account_master_presenter(id, Arc::clone(&presenter));

        let retrieved = registry.get_account_master_presenter(id);
        assert!(retrieved.is_some());
        assert_eq!(registry.total_count(), 1);
    }

    #[test]
    fn test_multiple_presenter_types() {
        let registry = PresenterRegistry::new();
        let search_id = Uuid::new_v4();
        let je_id = Uuid::new_v4();
        let am_id = Uuid::new_v4();

        // Register search presenter
        let (result_tx, _) = tokio::sync::mpsc::channel(100);
        let (error_tx, _) = tokio::sync::mpsc::channel(100);
        let (progress_tx, _) = tokio::sync::mpsc::channel(100);
        let (execution_time_tx, _) = tokio::sync::mpsc::channel(100);
        let search_presenter =
            Arc::new(SearchPresenter::new(result_tx, error_tx, progress_tx, execution_time_tx));
        registry.register_search_presenter(search_id, search_presenter);

        // Register journal entry presenter
        let (list_tx, _) = tokio::sync::mpsc::unbounded_channel();
        let (detail_tx, _) = tokio::sync::mpsc::unbounded_channel();
        let (result_tx, _) = tokio::sync::mpsc::unbounded_channel();
        let (progress_tx, _) = tokio::sync::mpsc::unbounded_channel();
        let je_presenter =
            Arc::new(JournalEntryPresenter::new(list_tx, detail_tx, result_tx, progress_tx));
        registry.register_journal_entry_presenter(je_id, je_presenter);

        // Register account master presenter
        let (tx, _) = tokio::sync::mpsc::unbounded_channel();
        let am_presenter = Arc::new(AccountMasterPresenter::new(tx));
        registry.register_account_master_presenter(am_id, am_presenter);

        assert_eq!(registry.total_count(), 3);
        assert!(registry.get_search_presenter(search_id).is_some());
        assert!(registry.get_journal_entry_presenter(je_id).is_some());
        assert!(registry.get_account_master_presenter(am_id).is_some());
    }

    #[test]
    fn test_clear_all() {
        let registry = PresenterRegistry::new();

        // Register multiple presenters
        let (result_tx, _) = tokio::sync::mpsc::channel(100);
        let (error_tx, _) = tokio::sync::mpsc::channel(100);
        let (progress_tx, _) = tokio::sync::mpsc::channel(100);
        let (execution_time_tx, _) = tokio::sync::mpsc::channel(100);
        let search_presenter =
            Arc::new(SearchPresenter::new(result_tx, error_tx, progress_tx, execution_time_tx));
        registry.register_search_presenter(Uuid::new_v4(), search_presenter);

        let (tx, _) = tokio::sync::mpsc::unbounded_channel();
        let am_presenter = Arc::new(AccountMasterPresenter::new(tx));
        registry.register_account_master_presenter(Uuid::new_v4(), am_presenter);

        assert_eq!(registry.total_count(), 2);

        // Clear all
        registry.clear_all();
        assert_eq!(registry.total_count(), 0);
    }

    #[test]
    fn test_concurrent_access() {
        let registry = Arc::new(PresenterRegistry::new());
        let mut handles = vec![];

        // Spawn multiple threads that register and unregister presenters
        for i in 0..10 {
            let registry_clone = Arc::clone(&registry);
            let handle = thread::spawn(move || {
                let id = Uuid::new_v4();

                // Register
                let (result_tx, _) = tokio::sync::mpsc::channel(100);
                let (error_tx, _) = tokio::sync::mpsc::channel(100);
                let (progress_tx, _) = tokio::sync::mpsc::channel(100);
                let (execution_time_tx, _) = tokio::sync::mpsc::channel(100);
                let presenter = Arc::new(SearchPresenter::new(
                    result_tx,
                    error_tx,
                    progress_tx,
                    execution_time_tx,
                ));
                registry_clone.register_search_presenter(id, presenter);

                // Retrieve
                let retrieved = registry_clone.get_search_presenter(id);
                assert!(retrieved.is_some());

                // Unregister
                if i % 2 == 0 {
                    registry_clone.unregister_search_presenter(id);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Should have 5 presenters left (odd numbered threads didn't unregister)
        assert_eq!(registry.total_count(), 5);
    }

    #[test]
    fn test_get_nonexistent_presenter() {
        let registry = PresenterRegistry::new();
        let id = Uuid::new_v4();

        // Try to get a presenter that was never registered
        assert!(registry.get_search_presenter(id).is_none());
        assert!(registry.get_journal_entry_presenter(id).is_none());
        assert!(registry.get_account_master_presenter(id).is_none());
    }
}
