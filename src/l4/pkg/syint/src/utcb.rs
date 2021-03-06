use l4::{
    ipc::{CapProvider, Serialiser},
    utcb::SndFlexPage as FlexPage,
    utcb::*
};
use std::mem::transmute;

use helpers::UtcbMrFake;
use utcb_cc::*;

tests! {
    fn msgtag_reimplementation_work_the_same_way() {
        use l4_sys::{l4_msgtag_w, msgtag};
        unsafe {
            assert_eq!(l4_msgtag_w(1, 2, 3, 4).raw, msgtag(1,2,3,4).raw);
            assert_eq!(l4_msgtag_w(4, 3, 2, 1).raw, msgtag(4,3,2,1).raw);
        }
    }

    fn msgtagwords_reimplementation_work_the_same_way() {
        use l4_sys::{msgtag, l4_msgtag_words_w, msgtag_words};
        let tag = msgtag(1, 2, 3, 4);
        unsafe {
            assert_eq!(l4_msgtag_words_w(tag),
                    msgtag_words(tag));
            let tag = msgtag(4, 3, 2, 1);
            assert_eq!(l4_msgtag_words_w(tag), msgtag_words(tag));
        }
    }

        // ToDo: make it use actual wrapper from libl4-wrapper, these functions need to be
        // re-exported as non-inline functions, first
//    fn msgtagflags_reimplementation_work_the_same_way() {
//        use l4_sys::{l4_msgtag, l4_msgtag_flags, msgtag_flags};
//        unsafe {
//            assert_eq!(l4_msgtag_flags(l4_msgtag(1, 2, 3, 4)),
//                    msgtag_flags(l4_msgtag(1,2,3,4)));
//            assert_eq!(l4_msgtag_flags(l4_msgtag(4, 3, 2, 1)),
//                    msgtag_flags(l4_msgtag(4,3,2,1)));
//        }
//    }

    fn serialisation_of_long_float_bool_same_as_cc() {
        let mut mr = UtcbMrFake::new();
        unsafe {
            mr.msg().write(987654u64).expect("Writing failed");
            mr.msg().write(3.14f32).expect("ToDo");
            mr.msg().write(true).expect("ToDo");
        }

        unsafe {
            let c_res = transmute::<*mut u8, *mut u64>(write_long_float_bool());
            assert_eq!(*c_res, mr.get(0));
            assert_eq!(*c_res.offset(1), mr.get(1));
        }
    }

    fn serialisation_of_bool_long_float_works() {
        let mut mr = UtcbMrFake::new();
        unsafe {
            mr.msg().write(true).expect("ToDo");
            mr.msg().write(987654u64).expect("Writing failed");
            mr.msg().write(3.14f32).expect("ToDo");
        }

        unsafe {
            let c_res = transmute::<*mut u8, *mut u64>(write_bool_long_float());
            assert_eq!(*c_res, mr.get(0));
            assert_eq!(*c_res.offset(1), mr.get(1));
            assert_eq!(*c_res.offset(2), mr.get(2));
        }
    }

    fn serialisation_of_u64_u32_u64_works() {
        let mut mr = UtcbMrFake::new();
        unsafe {
            mr.msg().write(42u64).expect("Writing failed");
            mr.msg().write(1337u32).expect("Writing failed");
            mr.msg().write(42u64).unwrap();
        }

        unsafe {
            let c_res = transmute::<*mut u8, *mut u64>(write_u64_u32_u64());
            assert_eq!(*c_res, mr.get(0));
            assert_eq!(*c_res.offset(1), mr.get(1));
            assert_eq!(*c_res.offset(2), mr.get(2));
            assert_eq!(*c_res.offset(3), 0);
        }
    }

    fn write_fpage_type_works() {
        let mut mr = UtcbMrFake::new();
        // write fpage with the C++ serialisation framework methods
        unsafe {
            let fp = ::l4_sys::l4_obj_fpage(0xcafebabe, 0,
                                        FpageRights::RWX.bits() as u8);
            write_cxx_snd_fpage(mr.mut_ptr(), fp);
            let c_mapopts = mr.get(0);
            let c_fpage = mr.get(1);
            // write the Rust version
            let rfp = FlexPage::new(fp.raw, 0, None);
            mr.msg().reset();
            FlexPage::write(rfp, &mut mr.msg()).unwrap();
            assert_eq!(mr.get(0), c_mapopts);
            assert_eq!(mr.get(1), c_fpage);
        }
    }

    fn serialising_u32_and_fpage_works_as_in_cc() {
        let mut mr = UtcbMrFake::new();
        // write fpage with the C++ serialisation framework methods
        unsafe {
            let c_side = transmute::<*mut u8, *mut u64>(write_u32_and_fpage());
            let fp = ::l4_sys::l4_obj_fpage(0xcafebabe, 0,
                                        FpageRights::RWX.bits() as u8);
            // write the Rust version
            mr.msg().write(314u32).unwrap();
            let rfp = FlexPage::new(fp.raw, 0, None);
            // must be written from the flex page to track it as item, not as word (see UTCB
            // terminologie)
            FlexPage::write(rfp, &mut mr.msg()).unwrap();

            assert_eq!(*c_side, mr.get(0));
            assert_eq!(*(c_side.offset(1)), mr.get(1));
            assert_eq!(*(c_side.offset(2)), mr.get(2));
            assert_eq!(mr.msg().words(), 1);
            assert_eq!(mr.msg().items(), 1);
        }
    }

    fn read_u64_u64_from_mr() {
        let mut mr = UtcbMrFake::new();
        mr.set(0, 284);
        mr.set(1, 989812);
        unsafe {
            assert_eq!(mr.msg().read::<u64>().unwrap(), 284);
            assert_eq!(mr.msg().read::<u64>().unwrap(), 989812);
        }
    }

    fn word_count_correct_for_u64() {
        let mut mr = UtcbMrFake::new();
        unsafe {
            mr.msg().write(0u64).expect("Writing failed");
            mr.msg().write(3u64).expect("Couldn't write to msg");
        }
        assert_eq!(mr.msg().words(), 2);
    }

    fn word_count_is_correctly_rounded_up() {
        let mut mr = UtcbMrFake::new();
        unsafe { mr.msg().write(08).expect("Writing failed"); }
        assert_eq!(mr.msg().words(), 1);
        unsafe { mr.msg().write(42u8).expect("Writing failed"); }
        assert_eq!(mr.msg().words(), 1);
        unsafe { mr.msg().write(42u8).expect("Writing failed"); }
        assert_eq!(mr.msg().words(), 1);
    }

    fn slices_are_written_correctly() {
        let mut mr = UtcbMrFake::new();
        unsafe {
            mr.msg().write_slice::<usize, u64>(&[1, 2, 3, 4, 5]).unwrap();
            mr.msg().reset();
            // length matches
            assert_eq!(mr.get(0), 5);
            for i in 1usize..6 {
                assert_eq!(mr.get(i), i as u64,
                    format!("expected {}, got {}", i + 1, mr.get(i)));
            }
        }
    }

    fn slices_are_read_correctly() {
        let mut mr = UtcbMrFake::new();
        unsafe {
            // we trust this function due to the test above
            mr.msg().write_slice::<usize, u64>(&[1, 2, 3, 4, 5]).unwrap();
            mr.msg().reset();
            let slice = mr.msg().read_slice::<usize, u64>().unwrap();
            // length matches
            // assert_eq!(slice.len(), 5);
            assert_eq!(slice.len(), 5);
            for i in 0usize..5 {
                assert_eq!(*slice.get(i).unwrap(), (i + 1) as u64,
                    format!("expected {}, got {}", i + 1, slice.get(i).unwrap()));
            }
        }
    }

    fn strings_are_correctly_serialised() {
        // expect length + strings with 0-byte (length counts 0-byte)
        let mut mr = UtcbMrFake::new();
        let string = String::from("Hello, world!");
        unsafe { mr.msg().write_str(&string).unwrap() }
        assert_eq!(mr.msg().words(), 3);
        mr.msg().reset();
        unsafe {
            assert_eq!(mr.msg().read::<usize>().unwrap(), 14usize);
            assert_eq!(mr.msg().read::<u8>().unwrap(), 'H' as u8);
            for _ in 0..12 {
                let _ = mr.msg().read::<u8>().unwrap();
            }
            assert_eq!(mr.msg().read::<u8>().unwrap(), 0);
        }
        ()
    }

    fn strings_are_correctly_deserialised() {
        // expect length + strings with 0-byte (length counts 0-byte)
        let mut mr = UtcbMrFake::new();
        let string = String::from("Hello, world!");
        // form the tedst above, we know that this works:
        unsafe { mr.msg().write_str(&string).unwrap() }
        mr.msg().reset();
        let mut buf_fake = l4::ipc::Bufferless;
        let greeting = unsafe {
            String::read(&mut mr.msg(), &mut buf_fake.access_buffers()).unwrap()
        };
        assert_eq!(greeting.len(), 13);
        assert_true!(greeting.starts_with("Hello"));
    }

    fn too_long_messages_are_detected() {
        use l4::error::*;
        let mut mr = UtcbMrFake::new();
        let msg = (0..200).fold(String::new(), |mut s, _| { s.push_str("abcdefghi "); s });
        let r = unsafe { mr.msg().write_str(&msg) };
        assert_eq!(r, Err::<(), Error>(Error::Generic(GenericErr::MsgTooLong)));
    }
    /*
    fn strings_without_0_bytes_deserialised();

    fn strings_with_0bytes_serialised();
    */
}
