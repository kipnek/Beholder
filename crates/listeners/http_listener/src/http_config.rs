use common::models::Route;


pub struct HttpListener {
    pub address: String,
    pub routes: Vec<Route>,
}

