

GRAMMARS = {

    'unsafe': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['return_reference'],
                ['some', 'direct_assign', 'some'],


                ['some', 'pointer_manip', 'some'],
                ['unsafe', '{', 'some', '}','some']
            ],
            'some': [['print_case'], [], ['stmt']],


            #* formato de las posibles asignaciones directas a punteros
            'direct_assign': [
                ['opt_dec', 'opt_pointer', 'IDENTIFIER', '=', 'expr', ';'],
                ['opt_dec', 'opt_pointer', 'IDENTIFIER', '=', 'expr', 'opt_method', '(', 'opt_expr', ')', ';'],
                # el siguiente caso se puede ver en tokio-master/tokio/src/sync/oneshot.rs entre otros
                ['opt_dec', 'opt_pointer', 'IDENTIFIER', '=', 'Some', '(', 'expr', ')', ';'],
                ['opt_dec', 'opt_pointer', 'IDENTIFIER', '=', 'Some', '(', '&', '*', 'IDENTIFIER', ')', ';'],
            ],

            #* formatode las posibles devoluciones de referencias
            'return_reference': [
                # el siguiente caso se puede ver en hyper-master/src/ffi/http_types.rs entre otros
                ['&', 'mut', '*', 'IDENTIFIER'],
                # el siguiente caso se puede ver en lucet-main/lucet-runtime/lucet-runtime-internals/src/instance/signals.rs entre otros
                ['*', 'IDENTIFIER'],
                ['&', 'IDENTIFIER', '.', 'IDENTIFIER', ';'],
                # el siguiente caso se puede ver en  hyper-master/benches/support/tokiort.rs entre otros
                ['IDENTIFIER', '.', 'opt_method', ';'],
                ['&', 'IDENTIFIER', '.', 'IDENTIFIER', 'opt_params', ';'],
                ['&', 'mut', 'IDENTIFIER', '.', 'IDENTIFIER', ';'],
                ['(', '&', 'IDENTIFIER', '.', 'IDENTIFIER', ',', '&mut', 'IDENTIFIER', '.', 'IDENTIFIER', ',', '&', 'IDENTIFIER', '.', 'IDENTIFIER', ')', ';'],
                ['*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', '=', 'Some', '(', 'IDENTIFIER', ')', ';'],
                # el siguiente caso se puede ver en  hyper-master/benches/support/task.rs entre otros
                ['*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', '=', 'IDENTIFIER', '(', 'expr', ')'],
                # el siguiente caso se usa en gxhash-main/src/hasher.rs entre otros
                ['let', 'IDENTIFIER', '=', '&', 'IDENTIFIER', 'as', '*', 'const', 'type', 'opt_as', ';', '*', 'IDENTIFIER', ';'],
                # el siguiente caso se usa en lucet-main/lucet-concurrency-tests/src/killswitch.rs entre otros
                ['let', 'IDENTIFIER', '=', 'IDENTIFIER', 'opt_method', 'comment', 'direct_assign'],
                ['expr']
            ],


            #* auxiliares:
            'opt_dec': [['let'], []],
            'opt_pointer': [['*'], []],
            'opt_lib_method': [[':',':' , 'IDENTIFIER', 'opt_params'], []],
            'opt_method': [['.' , 'IDENTIFIER', 'opt_params'], []],
            'opt_expr': [['expr'], {',','expr'}, []],
            'opt_as': [['as', '*', 'const', 'type'], []],
            'opt_params': [['(', 'expr' ,')'], {'(',')'}],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'expr_complex': [['&', '*', 'IDENTIFIER']],
            'type': [['IDENTIFIER']],
            'print_case': [
                ['println', '!', '(', 'STRING_LITERAL', 'opt_expr', ')', ';']
            ],
            'comment': [['COMMENT', 'comment'], []]

        }
    },



    'return_reference': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['&', 'IDENTIFIER', '.', 'IDENTIFIER', ';'],
                ['&mut', 'IDENTIFIER', '.', 'IDENTIFIER', ';'],
                ['let', 'IDENTIFIER', '=', 'IDENTIFIER', '(', ')', ';'],
                ['(', '&', 'IDENTIFIER', '.', 'IDENTIFIER', ',', '&mut', 'IDENTIFIER', '.', 'IDENTIFIER', ',', '&', 'IDENTIFIER', '.', 'IDENTIFIER', ')', ';'],
                ['*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', '=', 'Some', '(', 'IDENTIFIER', ')', ';'],
                ['*', 'IDENTIFIER', '=', 'Some', '(', 'IDENTIFIER', ')', ';'],
                ['let', 'IDENTIFIER', '=', '*', 'IDENTIFIER', ';'],
                ['let', 'IDENTIFIER', ':', 'type', '?=', 'unsafe', '{', 'expr', '}'],
                ['let', 'IDENTIFIER', '=', '&', 'IDENTIFIER', 'as', '*', 'const', 'type', 'opt_as', ';', '*', 'IDENTIFIER', ';'],
                ['unsafe', '{', 'expr', '}']
            ],
            'opt_as': [['as', '*', 'const', 'type'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    },


    'direct_assign': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['opt_init','*','IDENTIFIER', '=', 'expr', ';'],
                ['let', 'IDENTIFIER', '=', 'expr', 'opt_method', '(', 'opt_expr', ')', ';'],
                ['let', 'IDENTIFIER', '=', 'Some', '(', 'expr', 'opt_method', '(', 'expr', ')', ')', ';'],
                ['let', 'IDENTIFIER', '=', 'Some', '(', '&', '*', 'IDENTIFIER', ')', ';'],
                ['let', 'IDENTIFIER', ':', '*', 'mut', 'type', '=', 'Box',':', ':', 'into_raw', '(', 'IDENTIFIER', ')', ';', 'Ok', '(', 'Box',':', ':', 'from_raw', '(', 'IDENTIFIER', 'as', '*', 'mut', 'type', ')', ')']
            ],
            'opt_init': [['let'], []],
            'opt_method': [[':',':' , 'IDENTIFIER'], []],
            'opt_expr': [['expr'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    },


    'pointer_manip': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', '=', 'expr', ';'],
                ['let', 'IDENTIFIER', ':', '&mut', 'type', '=', '&mut', '*', 'IDENTIFIER', ';'],
                ['let', 'IDENTIFIER', ':', '*', 'mut', 'type', '=', 'expr', ';'],
                ['let', 'IDENTIFIER', ':', '*', 'mut', 'type', '=', '*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', ';'],
                ['*', 'unsafe', '{', '&mut', '*', 'IDENTIFIER', '}', 'opt_dots', '=', 'expr', ';'],
                #! el siguiente caso se usa en tokio-master/tokio/src/io/util/read_buf.rs entre otros
                ['let', 'IDENTIFIER', '=', '&mut', '*', '(', 'IDENTIFIER', 'as', '*', 'mut', 'type', ')', ';'],
                ['*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', '=', 'type', '::', 'IDENTIFIER', '(', '*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', ')', ';']
            ],
            'opt_dots': [['.', 'IDENTIFIER', 'opt_dots'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    },
    'string_manip': {
        'start': 'stmt',
        'rules': {
            'stmt': [['*', 'IDENTIFIER', '+=', 'STRING_LITERAL', ';']]
        }
    },

    'extern_func': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['extern', 'STRING_LITERAL', '{', 'fn', 'IDENTIFIER', '(', 'opt_params', ')', '->', 'type', ';', '}'],
                ['core::arch::x86_64::_mm_storeu_ps', '(', 'IDENTIFIER', '.as_mut().as_mut_ptr()', 'as', '*', 'mut', 'type', ',', 'expr', ')', ';'],
                ['*', '(', 'IDENTIFIER', '.as_mut().as_mut_ptr()', 'as', '*', 'mut', 'type', ')', '=', 'expr', ';']
            ],
            'opt_params': [['IDENTIFIER', ':', 'type', 'more_params'], []],
            'more_params': [[',', 'IDENTIFIER', ':', 'type', 'more_params'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    },
    'special_cases': {
        'start': 'stmt',
        'rules': {
            'stmt': [
                ['unsafe', '{', 'let', 'opt_mut', 'IDENTIFIER', '=', 'MaybeUninit', '::', '<', 'type', '>', '::uninit', '(', ')', ';', 'IDENTIFIER', '(', 'opt_expr', ',', 'std::ptr::null()', ',', 'IDENTIFIER', '.as_mut_ptr', '(', ')', ')', ';', 'IDENTIFIER', '.assume_init', '(', ')', ';', '}'],
                ['unsafe', '{', 'let', 'IDENTIFIER', '=', 'type', '::', '::data', '(', 'self.ptr', ')', ';', 'IDENTIFIER', '.as_ref()', 'opt_dots', '.fetch_add', '(', 'NUMBER', ',', 'IDENTIFIER', ')', ';', 'type', '{', 'ptr:', 'self.ptr,', 'pd:', 'PhantomData,', '}', '}']
            ],
            'opt_mut': [['mut'], []],
            'opt_expr': [['expr'], []],
            'opt_dots': [['.', 'IDENTIFIER', 'opt_dots'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }
    }
}
