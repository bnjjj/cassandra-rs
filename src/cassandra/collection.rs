use std::ffi::CString;

use cassandra_sys::CassCollection as _CassCollection;
use cassandra_sys::cass_collection_append_int32;
use cassandra_sys::cass_collection_append_int64;
use cassandra_sys::cass_collection_append_float;
use cassandra_sys::cass_collection_append_double;
use cassandra_sys::cass_collection_append_bool;
use cassandra_sys::cass_collection_append_bytes;
use cassandra_sys::cass_collection_append_uuid;
use cassandra_sys::cass_collection_append_string;
use cassandra_sys::cass_collection_append_inet;
#[allow(unused_imports)]
use cassandra_sys::cass_collection_append_decimal;
use cassandra_sys::cass_collection_append_collection;
use cassandra_sys::cass_collection_append_int16;
use cassandra_sys::cass_collection_append_int8;
use cassandra_sys::cass_collection_append_tuple;
use cassandra_sys::cass_collection_append_uint32;
use cassandra_sys::cass_collection_append_user_type;
use cassandra_sys::cass_collection_data_type;
use cassandra_sys::cass_collection_new;
use cassandra_sys::cass_collection_free;
use cassandra_sys::cass_collection_new_from_data_type;

use cassandra::iterator::CassIterator;
use cassandra::error::CassError;
use cassandra::tuple::Tuple;
use cassandra::user_type::UserType;
use cassandra::data_type::DataType;
use cassandra::data_type::ConstDataType;
use cassandra::uuid::Uuid;
use cassandra::inet::Inet;

#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub enum CassCollectionType {
    LIST = 32,
    MAP = 33,
    SET = 34,
}

pub trait CassCollection {
    type Value;

    fn inner(&self) -> *mut _CassCollection;
    ///Creates a new collection.
    fn new(item_count: u64) -> Self;

    ///Creates a new collection from an existing data type.
    fn new_from_data_type(value: DataType, item_count: u64) -> Self;

    ///Gets the data type of a collection.
    fn data_type(&self) -> ConstDataType {
        unsafe { ConstDataType(cass_collection_data_type(self.inner())) }
    }

    ///Appends a "tinyint" to the collection.
    fn append_int8(&mut self, value: i8) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_int8(self.inner(), value), None).wrap(self) }
    }

    /// Appends an "smallint" to the collection.
    fn append_int16(&mut self, value: i16) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_int16(self.inner(), value), None).wrap(self) }
    }

    ///Appends an "int" to the collection.
    fn append_int32(&mut self, value: i32) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_int32(self.inner(), value), None).wrap(self) }
    }

    ///Appends a "date" to the collection.
    fn append_uint32(&mut self, value: u32) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_uint32(self.inner(), value), None).wrap(self) }
    }

    ///Appends a "bigint", "counter", "timestamp" or "time" to the
    ///collection.
    fn append_int64(&mut self, value: i64) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_int64(self.inner(), value), None).wrap(self) }
    }

    ///Appends a "float" to the collection.
    fn append_float(&mut self, value: f32) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_float(self.inner(), value), None).wrap(self) }
    }

    ///Appends a "double" to the collection.
    fn append_double(&mut self, value: f64) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_double(self.inner(), value), None).wrap(self) }
    }

    ///Appends a "boolean" to the collection.
    fn append_bool(&mut self, value: bool) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_bool(self.inner(), if value { 1 } else { 0 }),
                             None)
                .wrap(self)
        }
    }

    ///Appends an "ascii", "text" or "varchar" to the collection.
    fn append_string(&mut self, value: &str) -> Result<&Self, CassError> {
        unsafe {
            let cstr = CString::new(value).unwrap();
            let result = cass_collection_append_string(self.inner(), cstr.as_ptr());
            CassError::build(result, None).wrap(self)
        }
    }

    /// Appends a "blob", "varint" or "custom" to the collection.
    fn append_bytes(&mut self, value: Vec<u8>) -> Result<&Self, CassError> {
        unsafe {
            let bytes = cass_collection_append_bytes(self.inner(), value[..].as_ptr(), value.len() as u64);
            CassError::build(bytes, None).wrap(self)
        }
    }

    ///Appends a "uuid" or "timeuuid"  to the collection.
    fn append_uuid(&mut self, value: Uuid) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_uuid(self.inner(), value.0), None).wrap(self) }
    }

    ///Appends an "inet" to the collection.
    fn append_inet(&mut self, value: Inet) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_inet(self.inner(), value.0), None).wrap(self) }
    }

    ///Appends a "list" to the collection.
    fn append_list(&mut self, value: List) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_collection(self.inner(), value.0),
                             None)
                .wrap(self)
        }
    }

    ///Appends a "set" to the collection.
    fn append_set(&mut self, value: Set) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_collection(self.inner(), value.0),
                             None)
                .wrap(self)
        }
    }

    ///Appends a "map" to the collection.
    fn append_map(&mut self, value: Map) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_collection(self.inner(), value.0),
                             None)
                .wrap(self)
        }
    }

    ///Appends a "tuple" to the collection.
    fn append_tuple(&mut self, value: Tuple) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_collection_append_tuple(self.inner(), value.0), None).wrap(self) }
    }

    ///Appends a "udt" to the collection.
    fn append_user_type(&mut self, value: UserType) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_collection_append_user_type(self.inner(), value.0),
                             None)
                .wrap(self)
        }
    }
}


pub struct List(pub *mut _CassCollection);


impl Drop for List {
    fn drop(&mut self) {
        unsafe { cass_collection_free(self.0) }
    }
}

impl CassCollection for List {
    type Value = _CassCollection;

    ///create a new list
    fn new(item_count: u64) -> Self {
        unsafe { List(cass_collection_new(CassCollectionType::LIST as u32, item_count)) }
    }

    fn new_from_data_type(value: DataType, item_count: u64) -> Self {
        unsafe { List(cass_collection_new_from_data_type(value.0, item_count)) }
    }

    fn inner(&self) -> *mut _CassCollection {
        self.0
    }
}

pub struct Set(pub *mut _CassCollection);

impl Drop for Set {
    fn drop(&mut self) {
        unsafe { cass_collection_free(self.inner()) }
    }
}

// impl CassIterator for Set{
//
// }

impl CassCollection for Set {
    type Value = _CassCollection;

    ///create a new list
    fn new(item_count: u64) -> Self {
        unsafe { Set(cass_collection_new(CassCollectionType::SET as u32, item_count)) }
    }

    fn new_from_data_type(value: DataType, item_count: u64) -> Self {
        unsafe { Set(cass_collection_new_from_data_type(value.0, item_count)) }
    }

    ///Helper method only
    fn inner(&self) -> *mut _CassCollection {
        self.0
    }
}


pub struct Map(pub *mut _CassCollection);

impl Drop for Map {
    fn drop(&mut self) {
        unsafe { cass_collection_free(self.0) }
    }
}

impl CassCollection for Map {
    type Value = _CassCollection;
    ///create a new list
    fn new(item_count: u64) -> Self {
        unsafe { Map(cass_collection_new(CassCollectionType::MAP as u32, item_count)) }
    }

    fn new_from_data_type(value: DataType, item_count: u64) -> Self {
        unsafe { Map(cass_collection_new_from_data_type(value.0, item_count)) }
    }

    fn inner(&self) -> *mut _CassCollection {
        self.0
    }
}