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

    @staticmethod
    def analyze(self, code_block):
        lexer = Lexer(code_block)
        tokens = lexer.tokenize()
        p = Parser(tokens)

        for type_u in self.types_of_unsafe:
            instance = p.parse(code_block, type_u)
            if instance:
                return type_u 