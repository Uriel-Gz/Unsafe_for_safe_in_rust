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
    
    def replace(self, string: str, matched_data):
        rule_index = matched_data['rule_index']
        matched_tokens = matched_data['matched_tokens']
        if rule_index == 0:
            return string.replace('&' + matched_tokens[1].value + '.' + matched_tokens[3].value + ';', matched_tokens[1].value + '.' + matched_tokens[3].value + ';')
        return string

    

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
    
    def replace(self, string: str, matched_data):
        # print(f'Matched data in PointerAssignment: {matched_data}')
        rule_index = matched_data['rule_index']
        matched_tokens = matched_data['matched_tokens']
        if rule_index == 0:
            return string.replace('*' + matched_tokens[1].value + ' = ' + matched_tokens[3].value + ';', 'mem.replace(' + matched_tokens[1].value + ', ' + matched_tokens[3].value + ')' + ';')
        return string
