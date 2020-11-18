use serde::{Deserialize, Serialize};

type DocumentUri = String;
type CodeActionKind = String;
type InitializedParams = ();
type DocumentSelector = Vec<DocumentFilter>;

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct Position {
    pub line: u16,
    pub character: u16
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct Location {
    pub uri: DocumentUri,
    pub range: Range,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct LocationLink {
    pub origin_selection_range: Range,
    pub target_uri: DocumentUri,
    pub target_range: Range,
    pub target_selection_rage: Range,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<u8>,
    pub code: Either<i32, String>,
    pub source: Option<String>,
    pub message: String,
    pub related_information: Vec<DiagnosticRelatedInformation>
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct DiagnosticRelatedInformation {
    pub location: Location,
    pub message: String,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TextDocumentEdit {
    pub text_document: VersionedTextDocumentIdentifier,
    pub edits: Vec<TextEdit>,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct TextDocumentIdentifier {
    pub uri: DocumentUri,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct VersionedTextDocumentIdentifier {
    #[serde(flatten)]
    pub text_document_identifier: TextDocumentIdentifier,
    pub version: Option<u32>
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct CreateFileOptions {
    pub overwrite: Option<bool>,
    pub ignore_if_exists: bool,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct RenameFileOptions {
    pub overwrite: Option<bool>,
    pub ignore_if_exists: Option<bool>,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct DeleteFileOptions {
    pub recursive: Option<bool>,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(tag="kind")]
pub enum ResourceOperation {
    Create {
        uri: DocumentUri,
        options: Option<CreateFileOptions>
    },
    Rename {
        old_uri: DocumentUri,
        new_uri: DocumentUri,
        options: Option<RenameFileOptions>,
    }
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct InitalizeParams<T, E> {
    pub process_id: Option<i32>,
    pub root_path: Option<String>,
    pub root_uri: Option<DocumentUri>,
    pub initialization_options: Option<T>,
    pub capabilities: ClientCapabilities<E>,
    pub trace: String,
    pub workspace_folder: Option<Vec<WorkspaceFolder>>
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct WorkspaceFolder {
    pub uri: DocumentUri,
    pub name: String,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ClientCapabilities<E> {
    pub workspace: Option<WorkspaceClientCapabilities>,
    pub text_document: Option<TextDocumentClientCapabilities>,
    pub experimental: Option<E>
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct WorkspaceEditClientCapabilities {
    pub document_changes: Option<bool>,
    pub resource_operations: Option<bool>,
    pub failure_handling: Option<bool>,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct HasDynamicRegistration {
    pub dynamic_registration: Option<bool>,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SymbolCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct WorkspaceClientCapabilities {
    pub apply_edit: Option<bool>,
    pub workspace_edit: Option<WorkspaceEditClientCapabilities>,
    pub did_change_configuration: Option<HasDynamicRegistration>,
    pub did_change_watched_files: Option<HasDynamicRegistration>,
    pub symbol: Option<SymbolCapabilities>,
    pub execute_command: Option<HasDynamicRegistration>,
    pub workspace_folders: Option<bool>,
    pub configuration: Option<bool>,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TextDocumentClientCapabilities {
    pub synchronization: Option<SynchronizationCapabilities>,
    pub completion: Option<CompletionCapabilities>,
    pub hover: Option<HoverCapabilities>,
    pub signature_help: Option<SignatureHelpCapabilities>,
    pub references: Option<HasDynamicRegistration>,
    pub document_highlight: Option<HasDynamicRegistration>,
    pub document_symbol: Option<DocumentSymbolCapabilities>,
    pub formatting: Option<HasDynamicRegistration>,
    pub range_formatting: Option<HasDynamicRegistration>,
    pub on_type_formatting: Option<HasDynamicRegistration>,
    pub declaration: Option<DeclarationCapabilities>,
    pub definition: Option<DeclarationCapabilities>,
    pub type_definition: Option<DeclarationCapabilities>,
    pub implementation: Option<DeclarationCapabilities>,
    pub code_action: Option<CodeActionCapabilities>,
    pub code_lens: Option<HasDynamicRegistration>,
    pub document_link: Option<HasDynamicRegistration>,
    pub color_provider: Option<HasDynamicRegistration>,
    pub rename: Option<RenameCapabilities>,
    pub publish_diagnostics: Option<HasDynamicRegistration>,
    pub folding_range: Option<FoldingRangeCapabilities>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SynchronizationCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>,
    pub will_save: Option<bool>,
    pub will_save_wait_until: Option<bool>,
    pub did_save: Option<bool>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CompletionCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>,
    pub completion_item: Option<CompletionItemCapabilities>,
    pub completion_item_kind: Option<CompletionItemKindValueSet>,
    pub context_support: Option<bool>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct HoverCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>,
    pub content_format: Option<Vec<MarkupKind>>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SignatureHelpCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>,
    pub signature_information: Option<SignatureInformationCapabilities>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct DocumentSymbolCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>,
    pub symbol_kind: Option<SymbolKindValueSet>
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct DeclarationCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>,
    pub link_support: Option<bool>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CodeActionCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>,
    pub code_action_literal_support: Option<CodeActionLiteralSupport>
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct RenameCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>,
    pub prepare_support: Option<bool>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct FoldingRangeCapabilities {
    #[serde(flatten)]
    pub dynamic_registration: Option<HasDynamicRegistration>,
    pub range_limit: Option<u32>,
    pub line_folding_onle: Option<bool>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CompletionItemCapabilities {
    pub snippet_support: Option<bool>,
    pub commit_characters_support: Option<bool>,
    pub documentation_format: Option<Vec<MarkupKind>>,
    pub deprecated_support: Option<bool>,
    pub preselect_support: Option<bool>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ValueSet<T> {
    pub value_set: T
}

type CompletionItemKindValueSet = ValueSet<Option<Vec<CompletionItemKind>>>;
type SymbolKindValueSet = ValueSet<Option<Vec<SymbolKind>>>;

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged="lowercase")]
pub enum MarkupKind {
    PlainText,
    Markdown,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SignatureInformationCapabilities {
    pub documentation_format: Option<Vec<MarkupKind>>,
    pub parameter_information: Option<ParameterInformation>
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CodeActionLiteralSupport {
    pub code_action_kind: ValueSet<Vec<CodeActionKind>>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ParameterInformation {
    pub label_offer_support: Option<bool>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct InitializeResult<E> {
    pub capabilities: ServerCapabilities<E>
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ServerCapabilities<T> {
    pub text_document_sync: Option<TextDocumentSyncOptions>,
    pub hover_provider: Option<bool>,
    pub completion_provider: Option<CompletionOptions>,
    pub signature_help_provider: Option<SignatureHelpOptions>,
    pub definition_provider: Option<bool>,
    pub type_definition_provider: Option<Either<bool, TypeDefinitionProvider>>,
    pub implementation_provider: Option<Either<bool, TypeDefinitionProvider>>,
    pub references_provider: Option<bool>,
    pub document_highlight_provider: Option<bool>,
    pub workspace_symbols_provider: Option<bool>,
    pub code_action_provider: Option<Either<bool, CodeActionOptions>>,
    pub code_lens_provider: Option<ResolveProviderOptions>,
    pub document_formatting_provider: Option<bool>,
    pub document_range_formatting_provider: Option<bool>,
    pub rename_provider: Option<Either<bool, RenameOptions>>,
    pub document_link_provider: Option<ResolveProviderOptions>,
    pub color_provider: Option<Either<bool, ColorProviderOptions>>,
    pub folding_range_provider: Option<Either<bool, FoldingRangeProviderOptions>>,
    pub declaration_provider: Option<Either<bool, FoldingRangeProviderOptions>>,
    pub execute_command_provider: Option<ExecuteCommandOptions>,
    pub workspace: Option<WorkspaceServerCapabilities>,
    pub experimental: Option<T>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TextDocumentSyncOptions {
    pub open_close: Option<bool>,
    pub change: Option<TextDocumentSyncKind>,
    pub will_save: Option<bool>,
    pub will_save_wait_until: Option<bool>,
    pub save: Option<SaveOptions>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SaveOptions {
    pub include_text: Option<bool>
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CompletionOptions {
    pub resolve_provider: Option<bool>,
    pub trigger_characters: Option<Vec<String>>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct SignatureHelpOptions {
    pub trigger_characters: Option<Vec<String>>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TypeDefinitionProvider {
    #[serde(flatten)]
    pub text_document_registration_options: TextDocumentRegistrationOptions,
    pub static_registration_options: StaticRegistrationOptions,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct TextDocumentRegistrationOptions {
    pub document_selector: Option<DocumentSelector>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct StaticRegistrationOptions {
    pub id: Option<String>,
}


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ResolveProviderOptions {
    pub resolve_provider: Option<bool>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct RenameOptions {
    pub prepare_provider: Option<bool>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct CodeActionOptions {
    pub code_action_kinds: Option<Vec<CodeActionKind>>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct FoldingRangeProviderOptions {
    #[serde(flatten)]
    pub text_document_registration_options: Option<TextDocumentRegistrationOptions>,
    #[serde(flatten)]
    pub static_registration_options: Option<StaticRegistrationOptions>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ColorProviderOptions {
    #[serde(flatten)]
    pub folding_range_provider_options: FoldingRangeProviderOptions,
    #[serde(flatten)]
    pub text_document_registration_options: TextDocumentRegistrationOptions,
    #[serde(flatten)]
    pub static_registration_options: StaticRegistrationOptions,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct ExecuteCommandOptions {
    pub commands: Vec<String>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct WorkspaceServerCapabilities {
    pub workspace_folder: Option<WorkspaceFolderOptions>,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct WorkspaceFolderOptions {
    pub supported: Option<bool>,
    pub change_notifications: Option<Either<String, bool>>
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct DocumentFilter {
    pub language: Option<String>,
    pub scheme: Option<String>,
    pub pattern: Option<String>,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum CompletionItemKind {
    Text = 1,
	Method = 2,
	Function = 3,
	Constructor = 4,
	Field = 5,
	Variable = 6,
	Class = 7,
	Interface = 8,
	Module = 9,
	Property = 10,
	Unit = 11,
	Value = 12,
	Enum = 13,
	Keyword = 14,
	Snippet = 15,
	Color = 16,
	File = 17,
	Reference = 18,
	Folder = 19,
	EnumMember = 20,
	Constant = 21,
	Struct = 22,
	Event = 23,
	Operator = 24,
	TypeParameter = 25,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}

#[derive(Copy, Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}