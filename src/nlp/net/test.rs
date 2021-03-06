pub struct HttpHeader{
//Standard 2 way headers
//Request Headers Standard
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
x_options: String,
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