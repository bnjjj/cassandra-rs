#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::mem;
use std::slice;
use std::str;

use cql_ffi::value::ValueType;
use cql_ffi::row::Row;

use cql_bindgen::CassResult as _CassandraResult;
use cql_bindgen::CassIterator as _CassIterator;
use cql_bindgen::cass_iterator_free;
use cql_bindgen::cass_iterator_next;
use cql_bindgen::cass_iterator_get_row;
use cql_bindgen::cass_result_free;
use cql_bindgen::cass_result_row_count;
use cql_bindgen::cass_result_column_count;
use cql_bindgen::cass_result_column_name;
use cql_bindgen::cass_result_column_type;
use cql_bindgen::cass_result_first_row;
use cql_bindgen::cass_result_has_more_pages;
use cql_bindgen::cass_iterator_from_result;
use cql_bindgen::cass_result_column_data_type;
use cql_bindgen::cass_result_paging_state_token;

pub struct CassandraResult(pub *const _CassandraResult);

impl Debug for CassandraResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, "Result row count: {:?}\n", self.row_count()));
        for row in self.iter() {
            try!(write!(f, "{:?}\n", row));
        }
        Ok(())
    }
}

impl Display for CassandraResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        try!(write!(f, "Result row count: {:?}\n", self.row_count()));
        for row in self.iter() {
            try!(write!(f, "{}\n", row));
        }
        Ok(())
    }
}

impl Drop for CassandraResult {
    fn drop(&mut self) {
        unsafe { self.free() }
    }
}

impl CassandraResult {
    unsafe fn free(&mut self) {
        cass_result_free(self.0)
    }

    pub fn row_count(&self) -> u64 {
        unsafe { cass_result_row_count(self.0) as u64 }
    }

    pub fn column_count(&self) -> u64 {
        unsafe { cass_result_column_count(self.0) as u64 }
    }

    // ~ result: *const CassandraResult, index: size_t,
    // ~ name: *mut *const ::libc::c_char,
    // ~ name_length: *mut size_t


    pub fn column_name(&self, index: u64) -> String {
        unsafe {
            let name = mem::zeroed();
            let name_length = mem::zeroed();
            cass_result_column_name(self.0, index, name, name_length);
            let slice = slice::from_raw_parts(name as *const u8, name_length as usize);
            str::from_utf8(slice).unwrap().to_owned()
        }
    }

    pub fn column_type(&self, index: u64) -> ValueType {
        unsafe { ValueType::build(cass_result_column_type(self.0, index)) }
    }

    pub fn first_row(&self) -> Option<Row> {
        unsafe {
            match self.row_count() {
                0 => None,
                _ => Some(Row(cass_result_first_row(self.0))),
            }
        }
    }

    pub fn has_more_pages(&self) -> bool {
        unsafe { cass_result_has_more_pages(self.0) > 0 }
    }

    pub fn iter(&self) -> ResultIterator {
        unsafe { ResultIterator(cass_iterator_from_result(self.0)) }
    }

}

pub struct ResultIterator(pub *mut _CassIterator);

impl Drop for ResultIterator {
    fn drop(&mut self) {
        unsafe { cass_iterator_free(self.0) }
    }
}

impl Iterator for ResultIterator {
    type Item = Row;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unsafe {
            match cass_iterator_next(self.0) {
                0 => None,
                _ => Some(self.get_row()),
            }
        }
    }
}

impl ResultIterator {
    pub fn get_row(&mut self) -> Row {
        unsafe { Row(cass_iterator_get_row(self.0)) }
    }

}

impl IntoIterator for CassandraResult {

    type Item = Row;
    type IntoIter = ResultIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// impl<'a> IntoIterator for &'a CassandraResult {
//    type Item = Row;
//    type IntoIter = ResultIterator;
//
//    fn into_iter(self) -> Self::IntoIter {unsafe{
//        ResultIterator(cass_iterator_from_result(self.0))
//    }}
// }
