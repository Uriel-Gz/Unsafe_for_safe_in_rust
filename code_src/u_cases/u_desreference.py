from u_cases.u_ipattern import IPattern


class PointerReturn(IPattern):

    rules = {
            'stmt': [
                ['&', 'IDENTIFIER', '.', 'IDENTIFIER', ';'],
                ['&mut', 'IDENTIFIER', '.', 'IDENTIFIER', ';'],
                ['(', '&', 'IDENTIFIER', '.', 'IDENTIFIER', ',', '&mut', 'IDENTIFIER', '.', 'IDENTIFIER', ',', '&', 'IDENTIFIER', '.', 'IDENTIFIER', ')', ';'],
                ['*', '(', 'IDENTIFIER', ')', '.', 'IDENTIFIER', '=', 'Some', '(', 'IDENTIFIER', ')', ';'],
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
                ['opt_dec', 'opt_pointer', 'IDENTIFIER', '=', 'expr', ';'],
                ['opt_dec', 'opt_pointer', 'IDENTIFIER', '=', 'expr', 'opt_method', '(', 'opt_expr', ')', ';'],
                ['opt_dec', 'opt_pointer', 'IDENTIFIER', '=', 'Some', '(', 'expr', ')', ';'],
                ['opt_dec', 'opt_pointer', 'IDENTIFIER', '=', 'Some', '(', '&', '*', 'IDENTIFIER', ')', ';'],
            ],
            'opt_dec': [['let'], []],
            'opt_pointer': [['*'], []],
            'opt_method': [[':',':' , 'IDENTIFIER'], []],
            'opt_expr': [['expr'], []],
            'expr': [['IDENTIFIER'], ['NUMBER'], ['STRING_LITERAL']],
            'expr_complex': [['&', '*', 'IDENTIFIER']],
            'type': [['IDENTIFIER']]
        }
    
    def replace(self, string: str, matched_data):
        # print(f'Matched data in PointerAssignment: {matched_data}')
        rule_index = matched_data['rule_index']
        matched_tokens = matched_data['matched_tokens']
        if rule_index == 0:
            return string.replace('*' + matched_tokens[1].value + ' = ' + matched_tokens[3].value + ';', 'mem::replace(' + matched_tokens[1].value + ', ' + matched_tokens[3].value + ')' + ';')
        elif rule_index == 1:
            aux = ''.join([token.value for token in matched_tokens[3:-1]])
            # print(f'Auxiliary expression in PointerAssignment: {aux}')
            return string.replace('let ' + matched_tokens[1].value + ' = ' + aux + ';', 'mem::replace(' + matched_tokens[1].value + ', ' + aux + ');')
        elif rule_index == 2:
            aux = ''.join([token.value for token in matched_tokens[3:-1]])
            # print(f'Auxiliary expression in PointerAssignment (Some case): {aux}')
            return string.replace('let ' + matched_tokens[1].value + ' = ' + aux + ';', 'mem::replace(' + matched_tokens[1].value + ', ' + aux + ');')
        elif rule_index == 3:
            aux = ''.join([token.value for token in matched_tokens[3:-1]])
            # print(f'Auxiliary expression in PointerAssignment (Some with deref case): {aux}')
            return string.replace('let ' + matched_tokens[1].value + ' = ' + aux + ';', 'mem::replace(' + matched_tokens[1].value + ', ' + aux + ');')
        return string
