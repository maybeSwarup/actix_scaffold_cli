use tera::{ Tera, Context };

fn get_tera() -> Tera {
    // In a real project, you might load from files. Here, we use in-memory templates for simplicity.
    let mut tera = Tera::default();
    tera.add_raw_template(
        "mod",
        "pub fn {{ name }}_logic() -> String {\n    \"{{ name }} logic\".into()\n}"
    ).unwrap();
    tera.add_raw_template(
        "route",
        "use actix_web::{get, HttpResponse, Responder};\n\n#[get(\"/{{ name }}\")]\npub async fn {{ name }}() -> impl Responder {\n    HttpResponse::Ok().body(\"{{ name }} works!\")\n}"
    ).unwrap();
    tera.add_raw_template(
        "middleware",
        "use actix_service::{Service, Transform};\nuse actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};\nuse futures_util::future::{ok, Ready};\nuse std::task::{Context, Poll};\nuse std::pin::Pin;\nuse std::rc::Rc;\n\npub struct {{ name }};\n\nimpl<S, B> Transform<S, ServiceRequest> for {{ name }}\nwhere\n    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,\n    S::Future: 'static,\n{\n    type Response = ServiceResponse<B>;\n    type Error = Error;\n    type Transform = {{ name }}Middleware<S>;\n    type InitError = ();\n    type Future = Ready<Result<Self::Transform, Self::InitError>>;\n\n    fn new_transform(&self, service: S) -> Self::Future {\n        ok({{ name }}Middleware {\n            service: Rc::new(service),\n        })\n    }\n}\n\npub struct {{ name }}Middleware<S> {\n    service: Rc<S>,\n}\n\nimpl<S, B> Service<ServiceRequest> for {{ name }}Middleware<S>\nwhere\n    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,\n    S::Future: 'static,\n{\n    type Response = ServiceResponse<B>;\n    type Error = Error;\n    type Future = Pin<Box<dyn futures_util::Future<Output = Result<Self::Response, Self::Error>>>>;\n\n    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {\n        self.service.poll_ready(cx)\n    }\n\n    fn call(&self, req: ServiceRequest) -> Self::Future {\n        println!(\"{{ name }} middleware hit: {}\", req.path());\n        let fut = self.service.call(req);\n        Box::pin(async move {\n            let res = fut.await?;\n            Ok(res)\n        })\n    }\n}\n"
    ).unwrap();
    tera.add_raw_template(
        "utils",
        "pub fn {{ name }}_helper() -> String {\n    \"Helper from {{ name }}\".into()\n}"
    ).unwrap();
    tera.add_raw_template(
        "test",
        "#[cfg(test)]\nmod tests {\n    use super::*;\n\n    #[test]\n    fn test_{{ name }}_basic() {\n        // TODO: Add real tests\n        assert!(true);\n    }\n}\n"
    ).unwrap();
    tera
}

pub fn get_template(kind: &str, name: &str) -> String {
    let tera = get_tera();
    let mut context = Context::new();
    context.insert("name", name);
    tera.render(kind, &context).unwrap()
}

pub fn get_test_template(_kind: &str, name: &str) -> String {
    let tera = get_tera();
    let mut context = Context::new();
    context.insert("name", name);
    tera.render("test", &context).unwrap()
}
