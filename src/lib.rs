#[macro_use]
extern crate cpython;
use cpython::{PyResult, Python};

//use targetlib;

fn clients_changed(_py: Python) -> PyResult<bool>{
    let result = targetlib::clients_changed();
    Ok(result)
}


fn get_client_info(_py: Python) -> PyResult<String> {
    let vec_result = targetlib::get_client_info();
    let mut str_result = String::new();
    for ci in vec_result {
        str_result.push_str(&ci.to_string());
        str_result.push_str(";");
    }
    Ok(str_result)
}


fn assign_spec(_py: Python, client_str: &str, spec_str: &str) -> PyResult<u32> {
    let spec = targetlib::CPSpec::from_str(spec_str).unwrap();
    let client = targetlib::CPClient::from_string(client_str);
    targetlib::assign_spec(&client, spec);
    Ok(0)
}

fn get_events(_py: Python, client_str: &str) -> PyResult<String> {
    let client = targetlib::CPClient::from_string(client_str);
    let events = targetlib::get_events(&client);
    let mut events_str = String::new();
    for e in events {
        events_str.push_str(&e.to_string());
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
