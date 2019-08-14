pub mod controllers;

//HTTP/1.1 支持的方法
enum RequestMethod{
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    TRACE,
    CONNECT,
    PATCH
}

struct Request{
    path: String,
    method: RequestMethod,

}
