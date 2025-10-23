from u_cases.u_ipattern import IPattern
import re


class PointerReturn(IPattern):

    rules = {
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
                ['expr']
            ],
            'opt_as': [['as', '*', 'const', 'type'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'type': [['IDENTIFIER']]
        }

    def match(self, code_line):
        pattern = re.compile(r'\*\s*(\w+);')
        match = pattern.search(code_line)
        if match:
            pointer_name = match.group(1)
            return pointer_name
        return None
    

    

class PointerAssignment(IPattern):

    rules = {
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

    def match(self, code_line):
        pattern = re.compile(r'\*\s*(\w+)\s*=\s*(.+);')
        match = pattern.search(code_line)
        if match:
            pointer_name = match.group(1)
            assigned_value = match.group(2)
            return pointer_name, assigned_value
        return None
    
    def replace(self, string: str):
        pass
