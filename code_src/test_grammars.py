from lexer import Lexer
from parsers import test_parser

# Primero, probar el lexer con un ejemplo simple
# code = 'let *x = 2;'
# lexer = Lexer(code)
# tokens = lexer.tokenize()
# print("Tokens for 'let x = ptr::read(y);':")
# for token in tokens:
#     print(token)
# print()

# Ejemplos de código para cada caso

examples = {
    'direct_assign': [
        '*var = 10;',
        'let *name = 2;',
        'let x = ptr::read(y);',
        'let x = Some(ptr::read(y));',
        'let x = Some(&*y);',
        'let raw_ptr: *mut i32 = Box::into_raw(x); Ok(Box::from_raw(raw_ptr as *mut i32))'
    ]
    # 'pointer_manip': [
    #     '*ptr.field = value;',
    #     'let x: &mut T = &mut *ptr;',
    #     'let x: *mut T = value;',
    #     'let x: *mut T = *ptr.field;',
    #     '*unsafe { &mut *ptr } = value;',
    #     'let x = &mut *(ptr as *mut T);',
    #     '*ptr.field = T::func(*ptr.field);'
    # ],
    # 'string_manip': [
    #     '*s += "hello";'
    # ],
    # 'value_return': [
    #     'let x = func();',
    #     '(&a.b, &mut a.c, &a.d);',
    #     '*ptr.field = Some(x);',
    #     '*ptr = Some(x);',
    #     'let x = *ptr;',
    #     'let x: T ?= unsafe { expr }',
    #     'let x = &ptr as *const T; *x;',
    #     'unsafe { expr }'
    # ],
    # 'extern_func': [
    #     'extern "C" { fn func(a: T) -> U; }',
    #     'core::arch::x86_64::_mm_storeu_ps(ptr.as_mut().as_mut_ptr() as *mut f32, value);',
    #     '*(ptr.as_mut().as_mut_ptr() as *mut T) = value;'
    # ],
    # 'special_cases': [
    #     'unsafe { let mut x = MaybeUninit::<T>::uninit(); func(None, std::ptr::null(), x.as_mut_ptr()); x.assume_init(); }',
    #     'unsafe { let x = T::data(self.ptr); x.as_ref().fetch_add(1, Ordering); T { ptr: self.ptr, pd: PhantomData, } }'
    # ]
}

if __name__ == "__main__":
    for grammar_name, codes in examples.items():
        print(f"Testing {grammar_name}:")
        for code in codes:
            result = test_parser(code, grammar_name)
            print(f"  '{code}' -> {result}")
        print()
