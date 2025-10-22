from u_ipattern import IPattern
import re


class PointerDereference(IPattern):
    def match(self, code_line):
        pattern = re.compile(r'\*\s*(\w+);')
        match = pattern.search(code_line)
        if match:
            pointer_name = match.group(1)
            return pointer_name
        return None
    

    

class PointerAssignment(IPattern):
    def match(self, code_line):
        pattern = re.compile(r'\*\s*(\w+)\s*=\s*(.+);')
        match = pattern.search(code_line)
        if match:
            pointer_name = match.group(1)
            assigned_value = match.group(2)
            return pointer_name, assigned_value
        return None
    
