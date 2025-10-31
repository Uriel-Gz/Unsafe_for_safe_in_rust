from u_cases.u_desreference import PointerReturn as ptrRtn
from u_cases.u_desreference import PointerAssignment as ptrAsgn

from lexer import Lexer
from parser import Parser

from grammars import GRAMMARS as G


class UnsafeAnalyzer:

    types_of_unsafe = [
        ptrRtn,
        ptrAsgn,
    ]

    def analyze(self, code_block):
        lexer = Lexer(code_block)
        tokens = lexer.tokenize()
        #! generalizar el lugar del unsafe
        tokens_ = tokens[2:] 
        p = Parser(tokens_)

        for type_u in self.types_of_unsafe:
            result = p.parse(type_u.rules)
            if result:
                return {'type_u': type_u, 'matched_data': result}
            
        return None