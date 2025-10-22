from lexer import Lexer
from grammars import GRAMMARS


class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0

    def current_token(self):
        if self.pos < len(self.tokens):
            return self.tokens[self.pos]
        return None

    def consume(self, expected_type=None):
        token = self.current_token()
        if token and (expected_type is None or token.type == expected_type):
            self.pos += 1
            return token
        return None

    def parse_rule(self, rule_name, grammar):
        rules = grammar['rules'][rule_name]
        for rule in rules:
            saved_pos = self.pos
            if self.match_sequence(rule):
                return True
            self.pos = saved_pos
        return False

    def match_sequence(self, sequence):
        for item in sequence:
            if isinstance(item, str):
                if item.startswith('opt_'):
                    # Optional rule
                    self.parse_rule(item, self.grammar)
                elif item in self.grammar['rules']:
                    if not self.parse_rule(item, self.grammar):
                        return False
                else:
                    # Terminal
                    token = self.consume()
                    # if token:
                    #     print(f"Matching terminal: expected {item}, got {token.type} and {token.value}")
                    if not token or token.type != item and token.value != item:
                        return False
            else:
                # For simplicity, assume terminals are strings
                token = self.consume()
                if not token or token.type != item and token.value != item:
                    return False
        return True

    def parse(self, grammar_name):
        self.grammar = GRAMMARS[grammar_name]
        return self.parse_rule(self.grammar['start'], self.grammar)


def test_parser(code, grammar_name):
    lexer = Lexer(code)
    tokens = lexer.tokenize()
    parser = Parser(tokens)
    return parser.parse(grammar_name)
