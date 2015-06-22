use bson::{Bson, Document};
use mongodb::client::wire_protocol::flags::{OpInsertFlags, OpQueryFlags,
                                            OpUpdateFlags};
use mongodb::client::wire_protocol::operations::Message;
use std::io::Write;
use std::net::TcpStream;

#[test]
fn insert_single_key_doc() {
    match TcpStream::connect("localhost:27017") {
        Ok(mut stream) => {
            let doc = doc! {
                "foo" => (Bson::FloatingPoint(42.0))
             };

            let docs = vec![doc];
            let flags = OpInsertFlags::no_flags();
            let name = "test.single_key".to_owned();
            let res = Message::with_insert(1, flags, name, docs);

            let cm = match res {
                Ok(message) => message,
                Err(_) => panic!("Could not create message!")
            };

            match cm.write(&mut stream) {
                Ok(_) => (),
                Err(s) => panic!("{}", s)
            };

            let doc = Document::new();
            let flags = OpQueryFlags::no_flags();
            let name = "test.single_key".to_owned();
            let res = Message::with_query(1, flags, name, 0, 0, doc, None);

            let cm = match res {
                Ok(message) => message,
                Err(s) => panic!("{}", s)
            };

            match cm.write(&mut stream) {
                Ok(_) => (),
                Err(s) => panic!("{}", s)
            };

            let reply = match Message::read(&mut stream) {
                Ok(m) => m,
                Err(s) => panic!("{}", s)
            };

            let docs = match reply {
                Message::OpReply { header: _, flags: _, cursor_id:_,
                                   starting_from: _, number_returned: _,
                                   documents: d } => d,
                _ => panic!("Invalid response read from server")
            };

            assert_eq!(docs.len() as i32, 1);

            match docs[0].get("foo") {
                Some(&Bson::FloatingPoint(42.0)) => (),
                _ => panic!("Wrong value returned!")
            };
        },
        Err(_) => {
            panic!("Could not connect to server")
        }
    }
}

#[test]
fn insert_multi_key_doc() {
    match TcpStream::connect("localhost:27017") {
        Ok(mut stream) => {
            let doc = doc! {
                "foo" => (Bson::FloatingPoint(42.0)),
                "bar" => (Bson::String("__z&".to_owned()))
            };

            let docs = vec![doc];
            let flags = OpInsertFlags::no_flags();
            let name = "test.multi_key".to_owned();
            let res = Message::with_insert(1, flags, name, docs);

            let cm = match res {
                Ok(message) => message,
                Err(s) => panic!("{}", s)
            };

            match cm.write(&mut stream) {
                Ok(_) => (),
                Err(s) => panic!("{}", s)
            };

            let doc = Document::new();
            let flags = OpQueryFlags::no_flags();
            let name = "test.multi_key".to_owned();
            let res = Message::with_query(1, flags, name, 0, 0, doc, None);

            let cm = match res {
                Ok(message) => message,
                Err(s) => panic!("{}", s)
            };

            match cm.write(&mut stream) {
                Ok(_) => (),
                Err(s) => panic!("{}", s)
            };

            let reply = match Message::read(&mut stream) {
                Ok(m) => m,
                Err(s) => panic!("{}", s)
            };

            let docs = match reply {
                Message::OpReply { header: _, flags: _, cursor_id:_,
                                   starting_from: _, number_returned: _,
                                   documents: d } => d,
                _ => panic!("Invalid response read from server")
            };

            assert_eq!(docs.len() as i32, 1);

            match docs[0].get("foo") {
                Some(&Bson::FloatingPoint(42.0)) => (),
                _ => panic!("Wrong value returned!")
            };

            match docs[0].get("bar") {
                Some(&Bson::String(ref s)) => assert_eq!(s, "__z&"),
                _ => panic!("Wrong value returned!")
            };
        },
        Err(_) => {
            panic!("Could not connect to server")
        }
    }
}

#[test]
fn insert_docs() {
    match TcpStream::connect("localhost:27017") {
        Ok(mut stream) => {
            let doc1 = doc! {
                "foo" => (Bson::FloatingPoint(42.0)),
                "bar" => (Bson::String("__z&".to_owned()))
            };

            let doc2 = doc! {
                "booyah" => (Bson::I32(23))
            };

            let docs = vec![doc1, doc2];
            let flags = OpInsertFlags::no_flags();
            let name = "test.multi_doc".to_owned();
            let res = Message::with_insert(1, flags, name, docs);

            let cm = match res {
                Ok(message) => message,
                Err(s) => panic!("{}", s)
            };

            match cm.write(&mut stream) {
                Ok(_) => (),
                Err(s) => panic!("{}", s)
            };

            let doc = Document::new();
            let flags = OpQueryFlags::no_flags();
            let name = "test.multi_doc".to_owned();
            let res = Message::with_query(1, flags, name, 0, 0, doc, None);

            let cm = match res {
                Ok(message) => message,
                Err(s) => panic!("{}", s)
            };

            match cm.write(&mut stream) {
                Ok(_) => (),
                Err(s) => panic!("{}", s)
            };


            let reply = match Message::read(&mut stream) {
                Ok(m) => m,
                Err(s) => panic!("{}", s)
            };

            let docs = match reply {
                Message::OpReply { header: _, flags: _, cursor_id:_,
                                   starting_from: _, number_returned: _,
                                   documents: d } => d,
                _ => panic!("Invalid response read from server")
            };

            assert_eq!(docs.len() as i32, 2);

            match docs[0].get("foo") {
                Some(&Bson::FloatingPoint(42.0)) => (),
                _ => panic!("Wrong value returned!")
            };

            match docs[0].get("bar") {
                Some(&Bson::String(ref s)) => assert_eq!(s, "__z&"),
                _ => panic!("Wrong value returned!")
            };

            match docs[1].get("booyah") {
                Some(&Bson::I32(23)) => (),
                _ => panic!("Wrong value returned!")
            };
        },
        Err(_) => {
            panic!("Could not connect to server")
        }
    }
}


#[test]
fn insert_update_then_query() {
    match TcpStream::connect("localhost:27017") {
        Ok(mut stream) => {
            let doc = doc! {
                "foo" => (Bson::FloatingPoint(42.0))
            };

            let docs = vec![doc];
            let flags = OpInsertFlags::no_flags();
            let name = "test.update".to_owned();
            let res = Message::with_insert(1, flags, name, docs);

            let cm = match res {
                Ok(message) => message,
                Err(_) => panic!("Could not create insert message!")
            };

            match cm.write(&mut stream) {
                Ok(_) => (),
                Err(s) => panic!("{}", s)
            };

            let selector = Document::new();

            let update = doc! {
                "foo" => (Bson::String("bar".to_owned()))
            };

            let flags = OpUpdateFlags::no_flags();
            let name = "test.update".to_owned();
            let res = Message::with_update(2, name, flags, selector, update);

            let cm = match res {
                Ok(message) => message,
                Err(_) => panic!("Could not create update message!")
            };

            match cm.write(&mut stream) {
                Ok(_) => (),
                Err(s) => panic!("{}", s)
            };

            let doc = Document::new();
            let flags = OpQueryFlags::no_flags();
            let name = "test.update".to_owned();
            let res = Message::with_query(3, flags, name, 0, 0, doc, None);

            let cm = match res {
                Ok(message) => message,
                Err(_) => panic!("Could not create query message!")
            };

            match cm.write(&mut stream) {
                Ok(_) => (),
                Err(s) => panic!("{}", s)
            };

            let reply = match Message::read(&mut stream) {
                Ok(m) => m,
                Err(s) => panic!("Could not read response: {}", s)
            };

            let docs = match reply {
                Message::OpReply { header: _, flags: _, cursor_id:_,
                                   starting_from: _, number_returned: _,
                                   documents: d } => d,
                _ => panic!("Invalid response read from server")
            };

            assert_eq!(docs.len() as i32, 1);

            match docs[0].get("foo") {
                Some(&Bson::String(ref s)) => assert_eq!(s, "bar"),
                _ => panic!("Wrong value returned!")
            };
        },
        Err(_) => {
            panic!("Could not connect to server")
        }
    }
}
