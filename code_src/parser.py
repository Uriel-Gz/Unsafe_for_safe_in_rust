# from lexer import Lexer
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
        rules = grammar[rule_name]
        for i, rule in enumerate(rules):
            saved_pos = self.pos
            matched_tokens = []
            if self.match_sequence_with_capture(rule, matched_tokens):
                return {'rule_index': i, 'rule': rule, 'matched_tokens': matched_tokens}
            self.pos = saved_pos
        return None

    def match_sequence_with_capture(self, sequence, matched_tokens):
        for item in sequence:
            if isinstance(item, str):
                if item.startswith('opt_'):
                    result = self.parse_rule(item, self.grammar)
                    if result:
                        matched_tokens.extend(result['matched_tokens'])
                    else:
                        return False
                elif item in self.grammar:
                    result = self.parse_rule(item, self.grammar)
                    if result:
                        matched_tokens.extend(result['matched_tokens'])
                    else:
                        return False
                else:
                    token = self.consume()
                    if not token or (token.type != item and token.value != item):
                        return False
                    matched_tokens.append(token)
            else:
                token = self.consume()
                if not token or (token.type != item and token.value != item):
                    return False
                matched_tokens.append(token)
        return True

    def match_sequence(self, sequence):
        for item in sequence:
            if isinstance(item, str):
                if item.startswith('opt_'):
                    if not self.parse_rule(item, self.grammar):
                        return False
                elif item in self.grammar:
                    if not self.parse_rule(item, self.grammar):
                        return False
                else:
                    token = self.consume()
                    # if token:
                    #     print(f"Matching terminal: expected {item}, got {token.type} and {token.value}")
                    if not token or token.type != item and token.value != item:
                        return False
            else:
                token = self.consume()
                if not token or token.type != item and token.value != item:
                    return False
        return True

    def parse(self, rules):
        self.grammar = rules
        return self.parse_rule('stmt', self.grammar)


