pub struct HttpHeader{
	//Standard 2 way headers: String,
	//Request Headers Standard: String,
	a_im: String,
	accept: String,
	accept_charset: String,
	accept_datetime: String,
	provisional: String,
	accept_encoding: String,
	accept_language: String,
	access_control_request_method: String,
	access_control_request_headers: String,
	authorization: String,
	cookie: String,
	expect: String,
	forwarded: String,
	from: String,
	host: String,
	http2_settings: String,
	if_match: String,
	if_modified_since: String,
	if_none_match: String,
	if_range: String,
	if_unmodified_since: String,
	max_forwards: String,
	origin: String,
	proxy_authorization: String,
	range: String,
	referer: String,
	te: String,
	user_agent: String,
	//request headers non_standard: String,
	upgrade_insecure_requests: String,
	x_requested_with: String,
	dnt: String,
	x_forwarded_for: String,
	x_forwarded_host: String,
	x_forwarded_proto: String,
	front_end_https: String,
	x_http_method_override: String,
	x_att_deviceid: String,
	x_wap_profile: String,
	proxy_connection: String,
	x_uidh: String,
	x_csrf_token: String,
	x_request_id,: String,
	save_data: String,
	//response headers standard: String,
	access_control_allow_headers: String,
	accept_patch: String,
	accept_ranges: String,
	age: String,
	allow: String,
	alt_svc: String,
	content_disposition: String,
	content_encoding: String,
	content_language: String,
	content_location: String,
	content_range: String,
	delta_base: String,
	e_tag: String,
	expires: String,
	im: String,
	last_modified: String,
	link: String,
	location: String,
	p3p: String,
	proxy_authenticate: String,
	public_key_pins: String,
	retry_after: String,
	server: String,
	set_cookie: String,
	strict_transport_security: String,
	trailer: String,
	transfer_encoding: String,
	tk: String,
	vary: String,
	www_authenticate: String,
	x_frame_options: String,
	//response headers non_standard: String,
	content_security_policy: String,
	x_content_security_policy: String,
	x_webkit_csp: String,
	refresh: String,
	status: String,
	timing_allow_origin: String,
	x_content_duration: String,
	x__options: String,
	x_powered_by: String,
	x_request_id: String,
	x_ua_compatible: String,
	x_xss_protection: String,
	//commons: String,
	cache_control: String,
	connection: String,
	content_length: String,
	content_md5: String,
	content_type: String,
	date: String,
	pragma: String,
	upgrade: String,
	via: String,
	warning: String,
	x_correlation_id: String
}

impl HttpHeader{
	//Constructor
	pub fn new() -> HttpHeader{
		a_im: String::new(),
		accept: String::new(),
		accept_charset: String::new(),
		accept_datetime: String::new(),
		provisional: String::new(),
		accept_encoding: String::new(),
		accept_language: String::new(),
		access_control_request_method: String::new(),
		access_control_request_headers: String::new(),
		authorization: String::new(),
		cookie: String::new(),
		expect: String::new(),
		forwarded: String::new(),
		from: String::new(),
		host: String::new(),
		http2_settings: String::new(),
		if_match: String::new(),
		if_modified_since: String::new(),
		if_none_match: String::new(),
		if_range: String::new(),
		if_unmodified_since: String::new(),
		max_forwards: String::new(),
		origin: String::new(),
		proxy_authorization: String::new(),
		range: String::new(),
		referer: String::new(),
		te: String::new(),
		user_agent: String::new(),
		//request headers non_standard: String::new(),
		upgrade_insecure_requests: String::new(),
		x_requested_with: String::new(),
		dnt: String::new(),
		x_forwarded_for: String::new(),
		x_forwarded_host: String::new(),
		x_forwarded_proto: String::new(),
		front_end_https: String::new(),
		x_http_method_override: String::new(),
		x_att_deviceid: String::new(),
		x_wap_profile: String::new(),
		proxy_connection: String::new(),
		x_uidh: String::new(),
		x_csrf_token: String::new(),
		x_request_id,: String::new(),
		save_data: String::new(),
		//response headers standard: String::new(),
		access_control_allow_headers: String::new(),
		accept_patch: String::new(),
		accept_ranges: String::new(),
		age: String::new(),
		allow: String::new(),
		alt_svc: String::new(),
		content_disposition: String::new(),
		content_encoding: String::new(),
		content_language: String::new(),
		content_location: String::new(),
		content_range: String::new(),
		delta_base: String::new(),
		e_tag: String::new(),
		expires: String::new(),
		im: String::new(),
		last_modified: String::new(),
		link: String::new(),
		location: String::new(),
		p3p: String::new(),
		proxy_authenticate: String::new(),
		public_key_pins: String::new(),
		retry_after: String::new(),
		server: String::new(),
		set_cookie: String::new(),
		strict_transport_security: String::new(),
		trailer: String::new(),
		transfer_encoding: String::new(),
		tk: String::new(),
		vary: String::new(),
		www_authenticate: String::new(),
		x_frame_options: String::new(),
		//response headers non_standard: String::new(),
		content_security_policy: String::new(),
		x_content_security_policy: String::new(),
		x_webkit_csp: String::new(),
		refresh: String::new(),
		status: String::new(),
		timing_allow_origin: String::new(),
		x_content_duration: String::new(),
		x__options: String::new(),
		x_powered_by: String::new(),
		x_request_id: String::new(),
		x_ua_compatible: String::new(),
		x_xss_protection: String::new(),
		//commons: String::new(),
		cache_control: String::new(),
		connection: String::new(),
		content_length: String::new(),
		content_md5: String::new(),
		content_type: String::new(),
		date: String::new(),
		pragma: String::new(),
		upgrade: String::new(),
		via: String::new(),
		warning: String::new(),
		x_correlation_id: String::new()
	}
	//Setters for struct or
	pub fn set_a_im(&mut self, val : String)	{ self.a_im = val; }
	pub fn set_accept(&mut self, val : String)	{ self.accept = val; }
	pub fn set_accept_charset(&mut self, val : String)	{ self.accept_charset = val; }
	pub fn set_accept_datetime(&mut self, val : String)	{ self.accept_datetime = val; }
	pub fn set_accept_encoding(&mut self, val : String)	{ self.accept_encoding = val; }
	pub fn set_accept_language(&mut self, val : String)	{ self.accept_language = val; }
	pub fn set_accept_patch(&mut self, val : String)	{ self.accept_patch = val; }
	pub fn set_accept_ranges(&mut self, val : String)	{ self.accept_ranges = val; }
	pub fn set_access_control_allow_headers(&mut self, val : String)	{ self.access_control_allow_headers = val; }
	pub fn set_access_control_request_headers(&mut self, val : String)	{ self.access_control_request_headers = val; }
	pub fn set_access_control_request_method(&mut self, val : String)	{ self.access_control_request_method = val; }
	pub fn set_age(&mut self, val : String)	{ self.age = val; }
	pub fn set_allow(&mut self, val : String)	{ self.allow = val; }
	pub fn set_alt_svc(&mut self, val : String)	{ self.alt_svc = val; }
	pub fn set_authorization(&mut self, val : String)	{ self.authorization = val; }
	pub fn set_cache_control(&mut self, val : String)	{ self.cache_control = val; }
	pub fn set_connection(&mut self, val : String)	{ self.connection = val; }
	pub fn set_content_disposition(&mut self, val : String)	{ self.content_disposition = val; }
	pub fn set_content_encoding(&mut self, val : String)	{ self.content_encoding = val; }
	pub fn set_content_language(&mut self, val : String)	{ self.content_language = val; }
	pub fn set_content_length(&mut self, val : String)	{ self.content_length = val; }
	pub fn set_content_location(&mut self, val : String)	{ self.content_location = val; }
	pub fn set_content_md5(&mut self, val : String)	{ self.content_md5 = val; }
	pub fn set_content_range(&mut self, val : String)	{ self.content_range = val; }
	pub fn set_content_security_policy(&mut self, val : String)	{ self.content_security_policy = val; }
	pub fn set_content_type(&mut self, val : String)	{ self.content_type = val; }
	pub fn set_cookie(&mut self, val : String)	{ self.cookie = val; }
	pub fn set_date(&mut self, val : String)	{ self.date = val; }
	pub fn set_delta_base(&mut self, val : String)	{ self.delta_base = val; }
	pub fn set_dnt(&mut self, val : String)	{ self.dnt = val; }
	pub fn set_e_tag(&mut self, val : String)	{ self.e_tag = val; }
	pub fn set_expect(&mut self, val : String)	{ self.expect = val; }
	pub fn set_expires(&mut self, val : String)	{ self.expires = val; }
	pub fn set_forwarded(&mut self, val : String)	{ self.forwarded = val; }
	pub fn set_from(&mut self, val : String)	{ self.from = val; }
	pub fn set_front_end_https(&mut self, val : String)	{ self.front_end_https = val; }
	pub fn set_host(&mut self, val : String)	{ self.host = val; }
	pub fn set_http2_settings(&mut self, val : String)	{ self.http2_settings = val; }
	pub fn set_if_match(&mut self, val : String)	{ self.if_match = val; }
	pub fn set_if_modified_since(&mut self, val : String)	{ self.if_modified_since = val; }
	pub fn set_if_none_match(&mut self, val : String)	{ self.if_none_match = val; }
	pub fn set_if_range(&mut self, val : String)	{ self.if_range = val; }
	pub fn set_if_unmodified_since(&mut self, val : String)	{ self.if_unmodified_since = val; }
	pub fn set_im(&mut self, val : String)	{ self.im = val; }
	pub fn set_last_modified(&mut self, val : String)	{ self.last_modified = val; }
	pub fn set_link(&mut self, val : String)	{ self.link = val; }
	pub fn set_location(&mut self, val : String)	{ self.location = val; }
	pub fn set_max_forwards(&mut self, val : String)	{ self.max_forwards = val; }
	pub fn set_origin(&mut self, val : String)	{ self.origin = val; }
	pub fn set_p3p(&mut self, val : String)	{ self.p3p = val; }
	pub fn set_pragma(&mut self, val : String)	{ self.pragma = val; }
	pub fn set_provisional(&mut self, val : String)	{ self.provisional = val; }
	pub fn set_proxy_authenticate(&mut self, val : String)	{ self.proxy_authenticate = val; }
	pub fn set_proxy_authorization(&mut self, val : String)	{ self.proxy_authorization = val; }
	pub fn set_proxy_connection(&mut self, val : String)	{ self.proxy_connection = val; }
	pub fn set_pub struct HttpHeader{(&mut self, val : String)	{ self.pub struct HttpHeader{ = val; }
	pub fn set_public_key_pins(&mut self, val : String)	{ self.public_key_pins = val; }
	pub fn set_range(&mut self, val : String)	{ self.range = val; }
	pub fn set_referer(&mut self, val : String)	{ self.referer = val; }
	pub fn set_refresh(&mut self, val : String)	{ self.refresh = val; }
	pub fn set_retry_after(&mut self, val : String)	{ self.retry_after = val; }
	pub fn set_save_data(&mut self, val : String)	{ self.save_data = val; }
	pub fn set_server(&mut self, val : String)	{ self.server = val; }
	pub fn set_set_cookie(&mut self, val : String)	{ self.set_cookie = val; }
	pub fn set_status(&mut self, val : String)	{ self.status = val; }
	pub fn set_strict_transport_security(&mut self, val : String)	{ self.strict_transport_security = val; }
	pub fn set_te(&mut self, val : String)	{ self.te = val; }
	pub fn set_timing_allow_origin(&mut self, val : String)	{ self.timing_allow_origin = val; }
	pub fn set_tk(&mut self, val : String)	{ self.tk = val; }
	pub fn set_trailer(&mut self, val : String)	{ self.trailer = val; }
	pub fn set_transfer_encoding(&mut self, val : String)	{ self.transfer_encoding = val; }
	pub fn set_upgrade(&mut self, val : String)	{ self.upgrade = val; }
	pub fn set_upgrade_insecure_requests(&mut self, val : String)	{ self.upgrade_insecure_requests = val; }
	pub fn set_user_agent(&mut self, val : String)	{ self.user_agent = val; }
	pub fn set_vary(&mut self, val : String)	{ self.vary = val; }
	pub fn set_via(&mut self, val : String)	{ self.via = val; }
	pub fn set_warning(&mut self, val : String)	{ self.warning = val; }
	pub fn set_www_authenticate(&mut self, val : String)	{ self.www_authenticate = val; }
	pub fn set_x__options(&mut self, val : String)	{ self.x__options = val; }
	pub fn set_x_att_deviceid(&mut self, val : String)	{ self.x_att_deviceid = val; }
	pub fn set_x_content_duration(&mut self, val : String)	{ self.x_content_duration = val; }
	pub fn set_x_content_security_policy(&mut self, val : String)	{ self.x_content_security_policy = val; }
	pub fn set_x_correlation_id(&mut self, val : String)	{ self.x_correlation_id = val; }
	pub fn set_x_csrf_token(&mut self, val : String)	{ self.x_csrf_token = val; }
	pub fn set_x_forwarded_for(&mut self, val : String)	{ self.x_forwarded_for = val; }
	pub fn set_x_forwarded_host(&mut self, val : String)	{ self.x_forwarded_host = val; }
	pub fn set_x_forwarded_proto(&mut self, val : String)	{ self.x_forwarded_proto = val; }
	pub fn set_x_frame_options(&mut self, val : String)	{ self.x_frame_options = val; }
	pub fn set_x_http_method_override(&mut self, val : String)	{ self.x_http_method_override = val; }
	pub fn set_x_powered_by(&mut self, val : String)	{ self.x_powered_by = val; }
	pub fn set_x_request_id(&mut self, val : String)	{ self.x_request_id = val; }
	pub fn set_x_request_id,(&mut self, val : String)	{ self.x_request_id, = val; }
	pub fn set_x_requested_with(&mut self, val : String)	{ self.x_requested_with = val; }
	pub fn set_x_ua_compatible(&mut self, val : String)	{ self.x_ua_compatible = val; }
	pub fn set_x_uidh(&mut self, val : String)	{ self.x_uidh = val; }
	pub fn set_x_wap_profile(&mut self, val : String)	{ self.x_wap_profile = val; }
	pub fn set_x_webkit_csp(&mut self, val : String)	{ self.x_webkit_csp = val; }
	pub fn set_x_xss_protection(&mut self, val : String)	{ self.x_xss_protection = val; }
//Getters for struct or
	pub fn get_a_im(&self) -> String	{ self.a_im }
	pub fn get_accept(&self) -> String	{ self.accept }
	pub fn get_accept_charset(&self) -> String	{ self.accept_charset }
	pub fn get_accept_datetime(&self) -> String	{ self.accept_datetime }
	pub fn get_accept_encoding(&self) -> String	{ self.accept_encoding }
	pub fn get_accept_language(&self) -> String	{ self.accept_language }
	pub fn get_accept_patch(&self) -> String	{ self.accept_patch }
	pub fn get_accept_ranges(&self) -> String	{ self.accept_ranges }
	pub fn get_access_control_allow_headers(&self) -> String	{ self.access_control_allow_headers }
	pub fn get_access_control_request_headers(&self) -> String	{ self.access_control_request_headers }
	pub fn get_access_control_request_method(&self) -> String	{ self.access_control_request_method }
	pub fn get_age(&self) -> String	{ self.age }
	pub fn get_allow(&self) -> String	{ self.allow }
	pub fn get_alt_svc(&self) -> String	{ self.alt_svc }
	pub fn get_authorization(&self) -> String	{ self.authorization }
	pub fn get_cache_control(&self) -> String	{ self.cache_control }
	pub fn get_connection(&self) -> String	{ self.connection }
	pub fn get_content_disposition(&self) -> String	{ self.content_disposition }
	pub fn get_content_encoding(&self) -> String	{ self.content_encoding }
	pub fn get_content_language(&self) -> String	{ self.content_language }
	pub fn get_content_length(&self) -> String	{ self.content_length }
	pub fn get_content_location(&self) -> String	{ self.content_location }
	pub fn get_content_md5(&self) -> String	{ self.content_md5 }
	pub fn get_content_range(&self) -> String	{ self.content_range }
	pub fn get_content_security_policy(&self) -> String	{ self.content_security_policy }
	pub fn get_content_type(&self) -> String	{ self.content_type }
	pub fn get_cookie(&self) -> String	{ self.cookie }
	pub fn get_date(&self) -> String	{ self.date }
	pub fn get_delta_base(&self) -> String	{ self.delta_base }
	pub fn get_dnt(&self) -> String	{ self.dnt }
	pub fn get_e_tag(&self) -> String	{ self.e_tag }
	pub fn get_expect(&self) -> String	{ self.expect }
	pub fn get_expires(&self) -> String	{ self.expires }
	pub fn get_forwarded(&self) -> String	{ self.forwarded }
	pub fn get_from(&self) -> String	{ self.from }
	pub fn get_front_end_https(&self) -> String	{ self.front_end_https }
	pub fn get_host(&self) -> String	{ self.host }
	pub fn get_http2_settings(&self) -> String	{ self.http2_settings }
	pub fn get_if_match(&self) -> String	{ self.if_match }
	pub fn get_if_modified_since(&self) -> String	{ self.if_modified_since }
	pub fn get_if_none_match(&self) -> String	{ self.if_none_match }
	pub fn get_if_range(&self) -> String	{ self.if_range }
	pub fn get_if_unmodified_since(&self) -> String	{ self.if_unmodified_since }
	pub fn get_im(&self) -> String	{ self.im }
	pub fn get_last_modified(&self) -> String	{ self.last_modified }
	pub fn get_link(&self) -> String	{ self.link }
	pub fn get_location(&self) -> String	{ self.location }
	pub fn get_max_forwards(&self) -> String	{ self.max_forwards }
	pub fn get_origin(&self) -> String	{ self.origin }
	pub fn get_p3p(&self) -> String	{ self.p3p }
	pub fn get_pragma(&self) -> String	{ self.pragma }
	pub fn get_provisional(&self) -> String	{ self.provisional }
	pub fn get_proxy_authenticate(&self) -> String	{ self.proxy_authenticate }
	pub fn get_proxy_authorization(&self) -> String	{ self.proxy_authorization }
	pub fn get_proxy_connection(&self) -> String	{ self.proxy_connection }
	pub fn get_pub struct HttpHeader{(&self) -> String	{ self.pub struct HttpHeader{ }
	pub fn get_public_key_pins(&self) -> String	{ self.public_key_pins }
	pub fn get_range(&self) -> String	{ self.range }
	pub fn get_referer(&self) -> String	{ self.referer }
	pub fn get_refresh(&self) -> String	{ self.refresh }
	pub fn get_retry_after(&self) -> String	{ self.retry_after }
	pub fn get_save_data(&self) -> String	{ self.save_data }
	pub fn get_server(&self) -> String	{ self.server }
	pub fn get_set_cookie(&self) -> String	{ self.set_cookie }
	pub fn get_status(&self) -> String	{ self.status }
	pub fn get_strict_transport_security(&self) -> String	{ self.strict_transport_security }
	pub fn get_te(&self) -> String	{ self.te }
	pub fn get_timing_allow_origin(&self) -> String	{ self.timing_allow_origin }
	pub fn get_tk(&self) -> String	{ self.tk }
	pub fn get_trailer(&self) -> String	{ self.trailer }
	pub fn get_transfer_encoding(&self) -> String	{ self.transfer_encoding }
	pub fn get_upgrade(&self) -> String	{ self.upgrade }
	pub fn get_upgrade_insecure_requests(&self) -> String	{ self.upgrade_insecure_requests }
	pub fn get_user_agent(&self) -> String	{ self.user_agent }
	pub fn get_vary(&self) -> String	{ self.vary }
	pub fn get_via(&self) -> String	{ self.via }
	pub fn get_warning(&self) -> String	{ self.warning }
	pub fn get_www_authenticate(&self) -> String	{ self.www_authenticate }
	pub fn get_x__options(&self) -> String	{ self.x__options }
	pub fn get_x_att_deviceid(&self) -> String	{ self.x_att_deviceid }
	pub fn get_x_content_duration(&self) -> String	{ self.x_content_duration }
	pub fn get_x_content_security_policy(&self) -> String	{ self.x_content_security_policy }
	pub fn get_x_correlation_id(&self) -> String	{ self.x_correlation_id }
	pub fn get_x_csrf_token(&self) -> String	{ self.x_csrf_token }
	pub fn get_x_forwarded_for(&self) -> String	{ self.x_forwarded_for }
	pub fn get_x_forwarded_host(&self) -> String	{ self.x_forwarded_host }
	pub fn get_x_forwarded_proto(&self) -> String	{ self.x_forwarded_proto }
	pub fn get_x_frame_options(&self) -> String	{ self.x_frame_options }
	pub fn get_x_http_method_override(&self) -> String	{ self.x_http_method_override }
	pub fn get_x_powered_by(&self) -> String	{ self.x_powered_by }
	pub fn get_x_request_id(&self) -> String	{ self.x_request_id }
	pub fn get_x_request_id,(&self) -> String	{ self.x_request_id, }
	pub fn get_x_requested_with(&self) -> String	{ self.x_requested_with }
	pub fn get_x_ua_compatible(&self) -> String	{ self.x_ua_compatible }
	pub fn get_x_uidh(&self) -> String	{ self.x_uidh }
	pub fn get_x_wap_profile(&self) -> String	{ self.x_wap_profile }
	pub fn get_x_webkit_csp(&self) -> String	{ self.x_webkit_csp }
	pub fn get_x_xss_protection(&self) -> String	{ self.x_xss_protection }
}