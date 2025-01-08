
<!-- 
<> Observaciones del analisis del código
<>-->

# Trabajo Final (Tesis)

## Observación de cantidad de bloques unsafe

 [21/11/2024] Al ejecutar la herramienta de identificacion de codigo unsafe sobre 6 repositorios extraidos de github, la cantidad de bloques con dicha caracteristicas registrados fue de 634, lo cual deja en evidencia su uso frecuente para diversas cuestiones.


## Corrector de código unsafe

[21/11/2024] El corrector esta en progreso de poder cambiar el codigo inseguro por uno que este verificado, de momento solo cambia por expresiones asignadas por el usuario.

## Librerias encontradas que trabajan con unsafe
1. ffi
2. mem
3. libc
4. std::os::unix::io::RawFd
5. std::os::unix::io::RawHandle
6) Pin

## Casos de uso de codigo unsafe
[22/12/2024] empezado

1) creacion de listas uso de libreria mem : 
    unsafe { std::mem::transmute_copy(&app.list_state) };
    unsafe { Self(mem::zeroed()) }

2) extraccion de buffer como puntero : 
    unsafe { backtrace_symbols_fd(buffer.as_ptr(), size, libc::STDERR_FILENO) };

3) escribir en un archivo lo que contiene un puntero: 
    unsafe { libc::write(libc::STDERR_FILENO, s.as_ptr().cast(), s.len()) };


3.1) prestacion de punteros de un archivo: 
    unsafe {
        std::os::unix::io::BorrowedFd::borrow_raw(std::os::unix::io::AsRawFd::as_raw_fd(self))
    }

    unsafe fn from_raw_fd(fd: std::os::unix::io::RawFd) -> Self {
        StdFile::from_raw_fd(fd).into()
    }   

4) declaracion de una pila?:  unsafe {
        // Collect return addresses
        let depth = libc::backtrace(stack_trace.as_mut_ptr(), MAX_FRAMES as i32);
        if depth == 0 {
            return
        }
        &stack_trace[0..depth as usize]
    };

5) instalar un elemento en una estructura: unsafe {
        let alt_stack_size: usize = min_sigstack_size() + 64 * 1024;
        let mut alt_stack: libc::stack_t = mem::zeroed();
        alt_stack.ss_sp = alloc(Layout::from_size_align(alt_stack_size, 1).unwrap()).cast();
        alt_stack.ss_size = alt_stack_size;
        libc::sigaltstack(&alt_stack, ptr::null_mut());

        let mut sa: libc::sigaction = mem::zeroed();
        sa.sa_sigaction = print_stack_trace as libc::sighandler_t;
        sa.sa_flags = libc::SA_NODEFER | libc::SA_RESETHAND | libc::SA_ONSTACK;
        libc::sigemptyset(&mut sa.sa_mask);
        libc::sigaction(libc::SIGSEGV, &sa, ptr::null_mut());
    }

6) extraccion de tamaño de un puntero: unsafe { libc::getauxval(AT_MINSIGSTKSZ) };

7) declaracion de estructuras compuestas : unsafe impl Send for LookupContext {}
#[derive(Debug)]
struct LookupContextInner {
    /// The target to lookup.
    target: discv5::Key<NodeKey>,
    /// The closest nodes
    closest_nodes: RefCell<BTreeMap<Distance, QueryNode>>,
    /// A listener for all the nodes retrieved in this lookup
    ///
    /// This is present if the lookup was triggered manually via [Discv4] and we want to return all
    /// the nodes once the lookup finishes.
    listener: Option<NodeRecordSender>,
}

8) llamada a methodos unsafe:  unsafe {
            match $self.get_unchecked_mut() {
                Self::EthOnly(l) => Pin::new_unchecked(l).$method($($args),+),
                Self::Satellite(r) => Pin::new_unchecked(r).$method($($args),+),
            }
    }

    LEGACY_ANALYZED_BYTECODE_ID => Self(unsafe {
                RevmBytecode::new_analyzed(
                    bytes,
                    buf.read_u64::<byteorder::BigEndian>().unwrap() as usize,
                    revm_primitives::JumpTable::from_slice(buf),
                )
            }),

    unsafe {
        let mut stat = Stat::new();
        mdbx_result(ffi::mdbx_env_stat_ex(
            self.env_ptr(),
            ptr::null(),
            stat.mdb_stat(),
            size_of::<Stat>(),
        ))?;
        Ok(stat)
    }

9) trasmutacion de punteros:  unsafe { std::mem::transmute(buf) }

10) iteracion sobre archivo: b.iter(|| unsafe {
            txn.txn_execute(|txn| {
                mdbx_cursor_open(txn, dbi, &mut cursor);
                let mut i = 0;
                let mut count = 0u32;

                while mdbx_cursor_get(cursor, &mut key, &mut data, MDBX_NEXT) == 0 {
                    i += key.iov_len + data.iov_len;
                    count += 1;
                }

                black_box(i);
                assert_eq!(count, n);
                mdbx_cursor_close(cursor);
            })
            .unwrap();
        })

    c.bench_function("bench_get_rand_raw", |b| {
        b.iter(|| unsafe {
            txn.txn_execute(|txn| {
                let mut i = 0;
                for key in &keys {
                    key_val.iov_len = key.len();
                    key_val.iov_base = key.as_bytes().as_ptr().cast_mut().cast();

                    mdbx_get(txn, dbi, &key_val, &mut data_val);

                    i += key_val.iov_len;
                }
                black_box(i);
            })
            .unwrap();
        })
    });

    c.bench_function("bench_put_rand_raw", |b| {
        b.iter(|| unsafe {
            let mut txn: *mut MDBX_txn = ptr::null_mut();
            env.with_raw_env_ptr(|env| {
                mdbx_txn_begin_ex(env, ptr::null_mut(), 0, &mut txn, ptr::null_mut());

                let mut i = 0;
                for (key, data) in &items {
                    key_val.iov_len = key.len();
                    key_val.iov_base = key.as_bytes().as_ptr().cast_mut().cast();
                    data_val.iov_len = data.len();
                    data_val.iov_base = data.as_bytes().as_ptr().cast_mut().cast();

                    i += mdbx_put(txn, dbi, &key_val, &mut data_val, 0);
                }
                assert_eq!(0, i);
                mdbx_txn_abort(txn);
            });
        })
    });

11) declaracion de funcione dentro de funciones:     
    #[doc(hidden)]
    unsafe fn decode_val<K: TransactionKind>(
        _: *const ffi::MDBX_txn,
        data_val: ffi::MDBX_val,
    ) -> Result<Self, Error> {
        let s = slice::from_raw_parts(data_val.iov_base as *const u8, data_val.iov_len);
        Self::decode(s)
    }


    #[doc(hidden)]
    unsafe fn decode_val<K: TransactionKind>(
        _txn: *const ffi::MDBX_txn,
        data_val: ffi::MDBX_val,
    ) -> Result<Self, Error> {
        let s = slice::from_raw_parts(data_val.iov_base as *const u8, data_val.iov_len);

        #[cfg(feature = "return-borrowed")]
        {
            Ok(Cow::Borrowed(s))
        }

        #[cfg(not(feature = "return-borrowed"))]
        {
            let is_dirty = (!K::IS_READ_ONLY) &&
                crate::error::mdbx_result(ffi::mdbx_is_dirty(_txn, data_val.iov_base))?;

            Ok(if is_dirty { Cow::Owned(s.to_vec()) } else { Cow::Borrowed(s) })
        }
    }


    unsafe fn decode_val<K: TransactionKind>(
        _: *const ffi::MDBX_txn,
        _: ffi::MDBX_val,
    ) -> Result<Self, Error> {
        Ok(())
    }

    const unsafe fn slice_to_val(slice: Option<&[u8]>) -> ffi::MDBX_val {
        match slice {
            Some(slice) => {
                ffi::MDBX_val { iov_len: slice.len(), iov_base: slice.as_ptr() as *mut c_void }
            }
            None => ffi::MDBX_val { iov_len: 0, iov_base: ptr::null_mut() },
        }
    }

    pub unsafe fn drop_db(&self, db: Database) -> Result<()> {
        mdbx_result(self.txn_execute(|txn| ffi::mdbx_drop(txn, db.dbi(), true))?)?;

        Ok(())
    }

    pub unsafe fn close_db(&self, db: Database) -> Result<()> {
        mdbx_result(ffi::mdbx_dbi_close(self.env().env_ptr(), db.dbi()))?;

        Ok(())
    }

    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        StdFile::from_raw_handle(handle).into()
    }


    #[inline]
    pub unsafe fn inner_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        self.buf
    }

    #[inline]
    pub unsafe fn unfilled_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        &mut self.buf[self.filled..]
    }

    #[inline]
    pub unsafe fn assume_init(&mut self, n: usize) {
        let new = self.filled + n;
        if new > self.initialized {
            self.initialized = new;
        }
    }

12) para partes de codigo de una funcion donde se crean punteros:

    unsafe {
        txn.txn_execute(|txn_ptr| {
            mdbx_result(ffi::mdbx_cursor_open(txn_ptr, dbi, &mut cursor))
        })??;
    }

    unsafe {
        let cursor = ffi::mdbx_cursor_create(ptr::null_mut());
        let res = ffi::mdbx_cursor_copy(other.cursor(), cursor);
        let s = Self { txn: other.txn.clone(), cursor };
        mdbx_result(res)?;
        Ok(s)
    }

    unsafe {
        let mut key_val = slice_to_val(key);
        let mut data_val = slice_to_val(data);
        let key_ptr = key_val.iov_base;
        let data_ptr = data_val.iov_base;
        self.txn.txn_execute(|txn| {
            let v = mdbx_result(ffi::mdbx_cursor_get(
                self.cursor,
                &mut key_val,
                &mut data_val,
                op,
            ))?;
            assert_ne!(data_ptr, data_val.iov_base);
            let key_out = {
                // MDBX wrote in new key
                if key_ptr == key_val.iov_base {
                    None
                } else {
                    Some(Key::decode_val::<K>(txn, key_val)?)
                }
            };
            let data_out = Value::decode_val::<K>(txn, data_val)?;
            Ok((key_out, data_out, v))
        })?
    }

    mdbx_result(unsafe {
        self.txn.txn_execute(|_| {
            ffi::mdbx_cursor_put(self.cursor, &key_val, &mut data_val, flags.bits())
        })?
    })?;

    .txn_execute_renew_on_timeout(|_| unsafe { ffi::mdbx_cursor_close(self.cursor) });

    pub(crate) fn new(env: Environment) -> Result<Self> {
        let mut txn: *mut ffi::MDBX_txn = ptr::null_mut();
        unsafe {
            mdbx_result(ffi::mdbx_txn_begin_ex(
                env.env_ptr(),
                ptr::null_mut(),
                K::OPEN_FLAGS,
                &mut txn,
                ptr::null_mut(),
            ))?;
            Ok(Self::new_from_ptr(env, txn))
        }
    }

    self.txn_execute(|txn| unsafe { ffi::mdbx_txn_id(txn) })

    self.txn_execute(|txn| unsafe {
        match ffi::mdbx_get(txn, dbi, &key_val, &mut data_val) {
            ffi::MDBX_SUCCESS => Key::decode_val::<K>(txn, data_val).map(Some),
            ffi::MDBX_NOTFOUND => Ok(None),
            err_code => Err(Error::from_err_code(err_code)),
        }
    })?

    mdbx_result(unsafe {
        ffi::mdbx_txn_commit_ex(txn, latency.mdb_commit_latency())
    })

    unsafe {
        self.txn_execute(|txn| {
            mdbx_result(ffi::mdbx_dbi_flags_ex(txn, db.dbi(), &mut flags, ptr::null_mut()))
        })??;
    }

    unsafe {
        let mut stat = Stat::new();
        self.txn_execute(|txn| {
            mdbx_result(ffi::mdbx_dbi_stat(txn, dbi, stat.mdb_stat(), size_of::<Stat>()))
        })??;
        Ok(stat)
    }
    
    nsafe {
        ffi::mdbx_txn_abort(txn);
    }

    mdbx_result(self.txn_execute(|txn| unsafe {
        ffi::mdbx_put(txn, dbi, &key_val, &mut data_val, flags.bits())
    })?)?;

    unsafe {
        mdbx_result(self.txn_execute(|txn| {
            ffi::mdbx_put(
                txn,
                db.dbi(),
                &key_val,
                &mut data_val,
                flags.bits() | ffi::MDBX_RESERVE,
            )
        })?)?;
        Ok(slice::from_raw_parts_mut(data_val.iov_base as *mut u8, data_val.iov_len))
    }

    if let Some(d) = data_val {
        unsafe { ffi::mdbx_del(txn, dbi, &key_val, &d) }
    } else {
        unsafe { ffi::mdbx_del(txn, dbi, &key_val, ptr::null()) }
    }

    mdbx_result(self.txn_execute(|txn| unsafe { ffi::mdbx_drop(txn, dbi, false) })?)?;

    mdbx_result(unsafe { mdbx_txn_renew(self.txn) })?;

    unsafe { Self(std::mem::zeroed()) }

    let res = mdbx_result(unsafe {
        ffi::mdbx_txn_begin_ex(
            env.0,
            parent.0,
            flags,
            &mut txn,
            ptr::null_mut(),
        )
    })

    sender.send(mdbx_result(unsafe { ffi::mdbx_txn_abort(tx.0) })).unwrap();

    mdbx_result(unsafe {
        ffi::mdbx_txn_commit_ex(tx.0, latency.mdb_commit_latency())
    })

    let result = mdbx_result(unsafe { ffi::mdbx_txn_reset(txn_ptr) });

    unsafe {
        txn.drop_db(db).unwrap();
    }

    let data_mmap = unsafe { Mmap::map(&data_file)? };
    let offset_mmap = unsafe { Mmap::map(&offset_file)? };

    unsafe {
        dest.set_len(dest.capacity());
    }

    unsafe {
        assert_eq!(kill(getpid(), signal), 0);
    }

    let s = unsafe { String::from_utf8_unchecked(Vec::from(s.as_ref())) };

    unsafe fn from_raw_handle(handle: RawHandle) -> Self;
    unsafe fn from_raw_socket(sock: RawSocket) -> Self;

    unsafe {
        BorrowedHandle::borrow_raw(
            AsRawHandle::as_raw_handle(self),
        )
    }

    unsafe {
        self.buf[self.initialized..end]
            .as_mut_ptr()
            .write_bytes(0, end - self.initialized);
    }

    unsafe {
        self.buf[self.filled..end]
            .as_mut_ptr()
            .cast::<u8>()
            .copy_from_nonoverlapping(buf.as_ptr(), amt);
    }

    unsafe { bytes::buf::UninitSlice::from_raw_parts_mut(ptr, len) }



13) para borrar un puntero:

        mdbx_result(unsafe {
            self.txn.txn_execute(|_| ffi::mdbx_cursor_del(self.cursor, flags.bits()))?
        })?;

14) para definicion de perfiles:

unsafe impl<K> Send for Cursor<K> where K: TransactionKind {}
unsafe impl<K> Sync for Cursor<K> where K: TransactionKind {}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

unsafe impl Send for TransactionPtr {}
unsafe impl Sync for TransactionPtr {}

unsafe impl Send for TxnPtr {}
unsafe impl Sync for TxnPtr {}

15) para la obtencion de valores de punteros raw:

    (cursor.rs tiene este bloque repetido en varias lugares)
                unsafe {
                    let result = cursor.txn.txn_execute(|txn| {
                        match ffi::mdbx_cursor_get(cursor.cursor(), &mut key, &mut data, op) {
                            ffi::MDBX_SUCCESS => {
                                let key = match Key::decode_val::<K>(txn, key) {
                                    Ok(v) => v,
                                    Err(e) => return Some(Err(e)),
                                };
                                let data = match Value::decode_val::<K>(txn, data) {
                                    Ok(v) => v,
                                    Err(e) => return Some(Err(e)),
                                };
                                Some(Ok((key, data)))
                            }
                            // MDBX_ENODATA can occur when the cursor was previously sought to a
                            // non-existent value, e.g. iter_from with a
                            // key greater than all values in the database.
                            ffi::MDBX_NOTFOUND | ffi::MDBX_ENODATA => None,
                            error => Some(Err(Error::from_err_code(error))),
                        }
                    });
                    match result {
                        Ok(result) => result,
                        Err(err) => Some(Err(err)),
                    }
                }

        unsafe { ffi::mdbx_cursor_get(cursor.cursor(), &mut key, &mut data, op) };

16) abrir archivos o pasar resultdos a una funcion:

    mdbx_result(unsafe { ffi::mdbx_dbi_open(txn_ptr, name_ptr, flags, &mut dbi) })

17) asignacion o concatenacion de strings:

    unsafe {
        *self.get_unchecked_mut().thing += ", world";
    }


split.rs tokio-master...