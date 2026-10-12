///Accessibility 协议域；所有调用共享同一个 core。
pub mod accessibility;
///Ads 协议域；所有调用共享同一个 core。
pub mod ads;
///Animation 协议域；所有调用共享同一个 core。
pub mod animation;
///Audits 协议域；所有调用共享同一个 core。
pub mod audits;
///Autofill 协议域；所有调用共享同一个 core。
pub mod autofill;
///BackgroundService 协议域；所有调用共享同一个 core。
pub mod background_service;
///BluetoothEmulation 协议域；所有调用共享同一个 core。
pub mod bluetooth_emulation;
///Browser 协议域；所有调用共享同一个 core。
pub mod browser;
///CacheStorage 协议域；所有调用共享同一个 core。
pub mod cache_storage;
///Cast 协议域；所有调用共享同一个 core。
pub mod cast;
///Console 协议域；所有调用共享同一个 core。
pub mod console;
///CrashReportContext 协议域；所有调用共享同一个 core。
pub mod crash_report_context;
///CSS 协议域；所有调用共享同一个 core。
pub mod css;
///Debugger 协议域；所有调用共享同一个 core。
pub mod debugger;
///DeviceAccess 协议域；所有调用共享同一个 core。
pub mod device_access;
///DeviceOrientation 协议域；所有调用共享同一个 core。
pub mod device_orientation;
///DigitalCredentials 协议域；所有调用共享同一个 core。
pub mod digital_credentials;
///DOM 协议域；所有调用共享同一个 core。
pub mod dom;
///DOMDebugger 协议域；所有调用共享同一个 core。
pub mod dom_debugger;
///DOMSnapshot 协议域；所有调用共享同一个 core。
pub mod dom_snapshot;
///DOMStorage 协议域；所有调用共享同一个 core。
pub mod dom_storage;
///Emulation 协议域；所有调用共享同一个 core。
pub mod emulation;
///EventBreakpoints 协议域；所有调用共享同一个 core。
pub mod event_breakpoints;
///Extensions 协议域；所有调用共享同一个 core。
pub mod extensions;
///FedCm 协议域；所有调用共享同一个 core。
pub mod fed_cm;
///Fetch 协议域；所有调用共享同一个 core。
pub mod fetch;
///FileSystem 协议域；所有调用共享同一个 core。
pub mod file_system;
///HeadlessExperimental 协议域；所有调用共享同一个 core。
pub mod headless_experimental;
///HeapProfiler 协议域；所有调用共享同一个 core。
pub mod heap_profiler;
///IndexedDB 协议域；所有调用共享同一个 core。
pub mod indexed_db;
///Input 协议域；所有调用共享同一个 core。
pub mod input;
///Inspector 协议域；所有调用共享同一个 core。
pub mod inspector;
///IO 协议域；所有调用共享同一个 core。
pub mod io;
///LayerTree 协议域；所有调用共享同一个 core。
pub mod layer_tree;
///Log 协议域；所有调用共享同一个 core。
pub mod log;
///Media 协议域；所有调用共享同一个 core。
pub mod media;
///Memory 协议域；所有调用共享同一个 core。
pub mod memory;
///Network 协议域；所有调用共享同一个 core。
pub mod network;
///Overlay 协议域；所有调用共享同一个 core。
pub mod overlay;
///Page 协议域；所有调用共享同一个 core。
pub mod page;
///Performance 协议域；所有调用共享同一个 core。
pub mod performance;
///PerformanceTimeline 协议域；所有调用共享同一个 core。
pub mod performance_timeline;
///Preload 协议域；所有调用共享同一个 core。
pub mod preload;
///Profiler 协议域；所有调用共享同一个 core。
pub mod profiler;
///PWA 协议域；所有调用共享同一个 core。
pub mod pwa;
///Runtime 协议域；所有调用共享同一个 core。
pub mod runtime;
///Schema 协议域；所有调用共享同一个 core。
pub mod schema;
///Security 协议域；所有调用共享同一个 core。
pub mod security;
///ServiceWorker 协议域；所有调用共享同一个 core。
pub mod service_worker;
///SmartCardEmulation 协议域；所有调用共享同一个 core。
pub mod smart_card_emulation;
///Storage 协议域；所有调用共享同一个 core。
pub mod storage;
///SystemInfo 协议域；所有调用共享同一个 core。
pub mod system_info;
///Target 协议域；所有调用共享同一个 core。
pub mod target;
///Tethering 协议域；所有调用共享同一个 core。
pub mod tethering;
///Tracing 协议域；所有调用共享同一个 core。
pub mod tracing;
///WebAudio 协议域；所有调用共享同一个 core。
pub mod web_audio;
///WebAuthn 协议域；所有调用共享同一个 core。
pub mod web_authn;
///WebMCP 协议域；所有调用共享同一个 core。
pub mod web_mcp;
///生成的薄 Domain 扩展；只借用 Session，不缓存或解释浏览器状态。
pub trait SessionDomains {
    ///借用 Session 创建 Accessibility 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.accessibility();
    ///# }
    ///```
    fn accessibility(&self) -> accessibility::Domain<'_>;
    ///借用 Session 创建 Ads 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.ads();
    ///# }
    ///```
    fn ads(&self) -> ads::Domain<'_>;
    ///借用 Session 创建 Animation 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.animation();
    ///# }
    ///```
    fn animation(&self) -> animation::Domain<'_>;
    ///借用 Session 创建 Audits 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.audits();
    ///# }
    ///```
    fn audits(&self) -> audits::Domain<'_>;
    ///借用 Session 创建 Autofill 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.autofill();
    ///# }
    ///```
    fn autofill(&self) -> autofill::Domain<'_>;
    ///借用 Session 创建 BackgroundService 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.background_service();
    ///# }
    ///```
    fn background_service(&self) -> background_service::Domain<'_>;
    ///借用 Session 创建 BluetoothEmulation 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.bluetooth_emulation();
    ///# }
    ///```
    fn bluetooth_emulation(&self) -> bluetooth_emulation::Domain<'_>;
    ///借用 Session 创建 Browser 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.browser();
    ///# }
    ///```
    fn browser(&self) -> browser::Domain<'_>;
    ///借用 Session 创建 CSS 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.css();
    ///# }
    ///```
    fn css(&self) -> css::Domain<'_>;
    ///借用 Session 创建 CacheStorage 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.cache_storage();
    ///# }
    ///```
    fn cache_storage(&self) -> cache_storage::Domain<'_>;
    ///借用 Session 创建 Cast 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.cast();
    ///# }
    ///```
    fn cast(&self) -> cast::Domain<'_>;
    ///借用 Session 创建 Console 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.console();
    ///# }
    ///```
    fn console(&self) -> console::Domain<'_>;
    ///借用 Session 创建 CrashReportContext 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.crash_report_context();
    ///# }
    ///```
    fn crash_report_context(&self) -> crash_report_context::Domain<'_>;
    ///借用 Session 创建 DOM 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.dom();
    ///# }
    ///```
    fn dom(&self) -> dom::Domain<'_>;
    ///借用 Session 创建 DOMDebugger 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.dom_debugger();
    ///# }
    ///```
    fn dom_debugger(&self) -> dom_debugger::Domain<'_>;
    ///借用 Session 创建 DOMSnapshot 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.dom_snapshot();
    ///# }
    ///```
    fn dom_snapshot(&self) -> dom_snapshot::Domain<'_>;
    ///借用 Session 创建 DOMStorage 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.dom_storage();
    ///# }
    ///```
    fn dom_storage(&self) -> dom_storage::Domain<'_>;
    ///借用 Session 创建 Debugger 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.debugger();
    ///# }
    ///```
    fn debugger(&self) -> debugger::Domain<'_>;
    ///借用 Session 创建 DeviceAccess 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.device_access();
    ///# }
    ///```
    fn device_access(&self) -> device_access::Domain<'_>;
    ///借用 Session 创建 DeviceOrientation 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.device_orientation();
    ///# }
    ///```
    fn device_orientation(&self) -> device_orientation::Domain<'_>;
    ///借用 Session 创建 DigitalCredentials 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.digital_credentials();
    ///# }
    ///```
    fn digital_credentials(&self) -> digital_credentials::Domain<'_>;
    ///借用 Session 创建 Emulation 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.emulation();
    ///# }
    ///```
    fn emulation(&self) -> emulation::Domain<'_>;
    ///借用 Session 创建 EventBreakpoints 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.event_breakpoints();
    ///# }
    ///```
    fn event_breakpoints(&self) -> event_breakpoints::Domain<'_>;
    ///借用 Session 创建 Extensions 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.extensions();
    ///# }
    ///```
    fn extensions(&self) -> extensions::Domain<'_>;
    ///借用 Session 创建 FedCm 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.fed_cm();
    ///# }
    ///```
    fn fed_cm(&self) -> fed_cm::Domain<'_>;
    ///借用 Session 创建 Fetch 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.fetch();
    ///# }
    ///```
    fn fetch(&self) -> fetch::Domain<'_>;
    ///借用 Session 创建 FileSystem 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.file_system();
    ///# }
    ///```
    fn file_system(&self) -> file_system::Domain<'_>;
    ///借用 Session 创建 HeadlessExperimental 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.headless_experimental();
    ///# }
    ///```
    fn headless_experimental(&self) -> headless_experimental::Domain<'_>;
    ///借用 Session 创建 HeapProfiler 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.heap_profiler();
    ///# }
    ///```
    fn heap_profiler(&self) -> heap_profiler::Domain<'_>;
    ///借用 Session 创建 IO 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.io();
    ///# }
    ///```
    fn io(&self) -> io::Domain<'_>;
    ///借用 Session 创建 IndexedDB 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.indexed_db();
    ///# }
    ///```
    fn indexed_db(&self) -> indexed_db::Domain<'_>;
    ///借用 Session 创建 Input 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.input();
    ///# }
    ///```
    fn input(&self) -> input::Domain<'_>;
    ///借用 Session 创建 Inspector 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.inspector();
    ///# }
    ///```
    fn inspector(&self) -> inspector::Domain<'_>;
    ///借用 Session 创建 LayerTree 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.layer_tree();
    ///# }
    ///```
    fn layer_tree(&self) -> layer_tree::Domain<'_>;
    ///借用 Session 创建 Log 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.log();
    ///# }
    ///```
    fn log(&self) -> log::Domain<'_>;
    ///借用 Session 创建 Media 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.media();
    ///# }
    ///```
    fn media(&self) -> media::Domain<'_>;
    ///借用 Session 创建 Memory 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.memory();
    ///# }
    ///```
    fn memory(&self) -> memory::Domain<'_>;
    ///借用 Session 创建 Network 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.network();
    ///# }
    ///```
    fn network(&self) -> network::Domain<'_>;
    ///借用 Session 创建 Overlay 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.overlay();
    ///# }
    ///```
    fn overlay(&self) -> overlay::Domain<'_>;
    ///借用 Session 创建 PWA 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.pwa();
    ///# }
    ///```
    fn pwa(&self) -> pwa::Domain<'_>;
    ///借用 Session 创建 Page 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.page();
    ///# }
    ///```
    fn page(&self) -> page::Domain<'_>;
    ///借用 Session 创建 Performance 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.performance();
    ///# }
    ///```
    fn performance(&self) -> performance::Domain<'_>;
    ///借用 Session 创建 PerformanceTimeline 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.performance_timeline();
    ///# }
    ///```
    fn performance_timeline(&self) -> performance_timeline::Domain<'_>;
    ///借用 Session 创建 Preload 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.preload();
    ///# }
    ///```
    fn preload(&self) -> preload::Domain<'_>;
    ///借用 Session 创建 Profiler 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.profiler();
    ///# }
    ///```
    fn profiler(&self) -> profiler::Domain<'_>;
    ///借用 Session 创建 Runtime 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.runtime();
    ///# }
    ///```
    fn runtime(&self) -> runtime::Domain<'_>;
    ///借用 Session 创建 Schema 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.schema();
    ///# }
    ///```
    fn schema(&self) -> schema::Domain<'_>;
    ///借用 Session 创建 Security 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.security();
    ///# }
    ///```
    fn security(&self) -> security::Domain<'_>;
    ///借用 Session 创建 ServiceWorker 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.service_worker();
    ///# }
    ///```
    fn service_worker(&self) -> service_worker::Domain<'_>;
    ///借用 Session 创建 SmartCardEmulation 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.smart_card_emulation();
    ///# }
    ///```
    fn smart_card_emulation(&self) -> smart_card_emulation::Domain<'_>;
    ///借用 Session 创建 Storage 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.storage();
    ///# }
    ///```
    fn storage(&self) -> storage::Domain<'_>;
    ///借用 Session 创建 SystemInfo 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.system_info();
    ///# }
    ///```
    fn system_info(&self) -> system_info::Domain<'_>;
    ///借用 Session 创建 Target 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.target();
    ///# }
    ///```
    fn target(&self) -> target::Domain<'_>;
    ///借用 Session 创建 Tethering 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.tethering();
    ///# }
    ///```
    fn tethering(&self) -> tethering::Domain<'_>;
    ///借用 Session 创建 Tracing 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.tracing();
    ///# }
    ///```
    fn tracing(&self) -> tracing::Domain<'_>;
    ///借用 Session 创建 WebAudio 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.web_audio();
    ///# }
    ///```
    fn web_audio(&self) -> web_audio::Domain<'_>;
    ///借用 Session 创建 WebAuthn 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.web_authn();
    ///# }
    ///```
    fn web_authn(&self) -> web_authn::Domain<'_>;
    ///借用 Session 创建 WebMCP 域视图，不分配远端资源。
    ///
    ///```no_run
    ///use ydp::SessionDomains as _;
    ///# fn view(session:&ydp_core::Session) {
    ///let domain = session.web_mcp();
    ///# }
    ///```
    fn web_mcp(&self) -> web_mcp::Domain<'_>;
}
impl SessionDomains for ydp_core::Session {
    fn accessibility(&self) -> accessibility::Domain<'_> {
        accessibility::Domain { session: self }
    }
    fn ads(&self) -> ads::Domain<'_> {
        ads::Domain { session: self }
    }
    fn animation(&self) -> animation::Domain<'_> {
        animation::Domain { session: self }
    }
    fn audits(&self) -> audits::Domain<'_> {
        audits::Domain { session: self }
    }
    fn autofill(&self) -> autofill::Domain<'_> {
        autofill::Domain { session: self }
    }
    fn background_service(&self) -> background_service::Domain<'_> {
        background_service::Domain { session: self }
    }
    fn bluetooth_emulation(&self) -> bluetooth_emulation::Domain<'_> {
        bluetooth_emulation::Domain { session: self }
    }
    fn browser(&self) -> browser::Domain<'_> {
        browser::Domain { session: self }
    }
    fn css(&self) -> css::Domain<'_> {
        css::Domain { session: self }
    }
    fn cache_storage(&self) -> cache_storage::Domain<'_> {
        cache_storage::Domain { session: self }
    }
    fn cast(&self) -> cast::Domain<'_> {
        cast::Domain { session: self }
    }
    fn console(&self) -> console::Domain<'_> {
        console::Domain { session: self }
    }
    fn crash_report_context(&self) -> crash_report_context::Domain<'_> {
        crash_report_context::Domain { session: self }
    }
    fn dom(&self) -> dom::Domain<'_> {
        dom::Domain { session: self }
    }
    fn dom_debugger(&self) -> dom_debugger::Domain<'_> {
        dom_debugger::Domain { session: self }
    }
    fn dom_snapshot(&self) -> dom_snapshot::Domain<'_> {
        dom_snapshot::Domain { session: self }
    }
    fn dom_storage(&self) -> dom_storage::Domain<'_> {
        dom_storage::Domain { session: self }
    }
    fn debugger(&self) -> debugger::Domain<'_> {
        debugger::Domain { session: self }
    }
    fn device_access(&self) -> device_access::Domain<'_> {
        device_access::Domain { session: self }
    }
    fn device_orientation(&self) -> device_orientation::Domain<'_> {
        device_orientation::Domain { session: self }
    }
    fn digital_credentials(&self) -> digital_credentials::Domain<'_> {
        digital_credentials::Domain { session: self }
    }
    fn emulation(&self) -> emulation::Domain<'_> {
        emulation::Domain { session: self }
    }
    fn event_breakpoints(&self) -> event_breakpoints::Domain<'_> {
        event_breakpoints::Domain { session: self }
    }
    fn extensions(&self) -> extensions::Domain<'_> {
        extensions::Domain { session: self }
    }
    fn fed_cm(&self) -> fed_cm::Domain<'_> {
        fed_cm::Domain { session: self }
    }
    fn fetch(&self) -> fetch::Domain<'_> {
        fetch::Domain { session: self }
    }
    fn file_system(&self) -> file_system::Domain<'_> {
        file_system::Domain { session: self }
    }
    fn headless_experimental(&self) -> headless_experimental::Domain<'_> {
        headless_experimental::Domain { session: self }
    }
    fn heap_profiler(&self) -> heap_profiler::Domain<'_> {
        heap_profiler::Domain { session: self }
    }
    fn io(&self) -> io::Domain<'_> {
        io::Domain { session: self }
    }
    fn indexed_db(&self) -> indexed_db::Domain<'_> {
        indexed_db::Domain { session: self }
    }
    fn input(&self) -> input::Domain<'_> {
        input::Domain { session: self }
    }
    fn inspector(&self) -> inspector::Domain<'_> {
        inspector::Domain { session: self }
    }
    fn layer_tree(&self) -> layer_tree::Domain<'_> {
        layer_tree::Domain { session: self }
    }
    fn log(&self) -> log::Domain<'_> {
        log::Domain { session: self }
    }
    fn media(&self) -> media::Domain<'_> {
        media::Domain { session: self }
    }
    fn memory(&self) -> memory::Domain<'_> {
        memory::Domain { session: self }
    }
    fn network(&self) -> network::Domain<'_> {
        network::Domain { session: self }
    }
    fn overlay(&self) -> overlay::Domain<'_> {
        overlay::Domain { session: self }
    }
    fn pwa(&self) -> pwa::Domain<'_> {
        pwa::Domain { session: self }
    }
    fn page(&self) -> page::Domain<'_> {
        page::Domain { session: self }
    }
    fn performance(&self) -> performance::Domain<'_> {
        performance::Domain { session: self }
    }
    fn performance_timeline(&self) -> performance_timeline::Domain<'_> {
        performance_timeline::Domain { session: self }
    }
    fn preload(&self) -> preload::Domain<'_> {
        preload::Domain { session: self }
    }
    fn profiler(&self) -> profiler::Domain<'_> {
        profiler::Domain { session: self }
    }
    fn runtime(&self) -> runtime::Domain<'_> {
        runtime::Domain { session: self }
    }
    fn schema(&self) -> schema::Domain<'_> {
        schema::Domain { session: self }
    }
    fn security(&self) -> security::Domain<'_> {
        security::Domain { session: self }
    }
    fn service_worker(&self) -> service_worker::Domain<'_> {
        service_worker::Domain { session: self }
    }
    fn smart_card_emulation(&self) -> smart_card_emulation::Domain<'_> {
        smart_card_emulation::Domain { session: self }
    }
    fn storage(&self) -> storage::Domain<'_> {
        storage::Domain { session: self }
    }
    fn system_info(&self) -> system_info::Domain<'_> {
        system_info::Domain { session: self }
    }
    fn target(&self) -> target::Domain<'_> {
        target::Domain { session: self }
    }
    fn tethering(&self) -> tethering::Domain<'_> {
        tethering::Domain { session: self }
    }
    fn tracing(&self) -> tracing::Domain<'_> {
        tracing::Domain { session: self }
    }
    fn web_audio(&self) -> web_audio::Domain<'_> {
        web_audio::Domain { session: self }
    }
    fn web_authn(&self) -> web_authn::Domain<'_> {
        web_authn::Domain { session: self }
    }
    fn web_mcp(&self) -> web_mcp::Domain<'_> {
        web_mcp::Domain { session: self }
    }
}
///只读生成来源信息；不包含 runtime 方法查找表。
pub struct GeneratedMetadata {
    ///生成契约版本。
    pub generated_contract_version: u32,
    ///生成器包版本。
    pub generator_version: &'static str,
    ///绑定 crate 名称。
    pub binding_package: &'static str,
    ///每个输入自己的路径、版本和摘要。
    pub inputs: &'static [SourceMetadata],
    ///配置和说明目录的 SHA-256。
    pub configuration_sha256: &'static str,
}
///一个固定源文件的独立 provenance；不编造浏览器版本。
pub struct SourceMetadata {
    ///manifest 相对路径。
    pub path: &'static str,
    ///原始字节 SHA-256。
    pub sha256: &'static str,
    ///源协议主版本字符串。
    pub version_major: &'static str,
    ///源协议次版本字符串。
    pub version_minor: &'static str,
}
///固定生成快照的只读元数据；所有字符串静态存储，无运行时加载或分配。
pub const GENERATED_METADATA: GeneratedMetadata = GeneratedMetadata {
    generated_contract_version: 1,
    generator_version: "0.1.0-alpha.2",
    binding_package: "ydp",
    inputs: &[
        SourceMetadata {
            path: "json/browser_protocol.json",
            sha256: "7a707daef67560ed7e3ab01c30afc324701abb7a5ea71db3b6c32bb162e5d8be",
            version_major: "1",
            version_minor: "3",
        },
        SourceMetadata {
            path: "json/js_protocol.json",
            sha256: "cf124bf143bf361a7900bcfeea3926c1cfcc1da36d83756c228c43d84321539a",
            version_major: "1",
            version_minor: "3",
        },
    ],
    configuration_sha256: "2bd790c6d66afae87a104a22980e9283b2429d36bba87b91c731e482466d28bd",
};
