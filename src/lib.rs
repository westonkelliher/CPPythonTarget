#[macro_use]
extern crate cpython;
use cpython::{PyResult, Python};
extern crate serde_json;

extern crate targetlib;
use targetlib::{CPClient, CPSpec};
// TODO:
// we are converting back and forth between JSON and Rust Struct multiple
// times. May want to just keep things in JSON throughout.

fn clients_changed(_py: Python) -> PyResult<bool>{
    let result = targetlib::clients_changed();
    Ok(result)
}


fn get_client_info(_py: Python) -> PyResult<String> {
    let vec_result = targetlib::get_client_info();
    let mut str_result = String::new();
    for ci in vec_result {
	let ci_str = serde_json::to_string(&ci)
	    .unwrap_or_else(|_| panic!("Failed to json serialize {:?}", &ci));
	str_result.push_str(&ci_str);
        str_result.push_str("\n");
    }
    Ok(str_result)
}


fn assign_spec(_py: Python, client_str: &str, spec_str: &str) -> PyResult<u32> {
    let client: CPClient = serde_json::from_str(client_str)
	.unwrap_or_else(|_| panic!("Failed to json deserialize <{}>", client_str));
    let spec: CPSpec = serde_json::from_str(spec_str).unwrap();
    //.unwrap_or_else(|_| panic!("Failed to json deserialize <{}>", spec_str));
    targetlib::assign_spec(&client, spec);
    Ok(0)
}

fn get_events(_py: Python, client_str: &str) -> PyResult<String> {
    let client: CPClient = serde_json::from_str(client_str)
	.unwrap_or_else(|_| panic!("Failed to json deserialize <{}>", client_str));
    let events = targetlib::get_events(&client);
    let mut events_str = String::new();
    for e in events {
	let e_str = serde_json::to_string(&e)
	    .unwrap_or_else(|_| panic!("Failed to json serialize {:?}", &e));
        events_str.push_str(&e_str);
	events_str.push_str("\n");
    }
    Ok(events_str)
}


py_module_initializer!(control_pad_target, initcontrol_pad_target, Pyinit_control_pad_target, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    //m.add(py, "get_events", py_fn!(py, get_events()))?;
    //m.add(py, "initiate", py_fn!(py, initiate(specstr: &str)))?;
    m.add(py, "clients_changed", py_fn!(py, clients_changed()))?;
    m.add(py, "get_client_info", py_fn!(py, get_client_info()))?;
    m.add(py, "assign_spec", py_fn!(py, assign_spec(client_str: &str, spec_str: &str)))?;
    m.add(py, "get_events", py_fn!(py, get_events(client_str: &str)))?;
    Ok(())
});
