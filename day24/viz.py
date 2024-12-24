import sys
import graphviz

dot = graphviz.Digraph()
for line in sys.stdin:
    line = line.rstrip()
    if line == "":
        break
    (s, val) = line.split(": ")
    dot.node(s)

for line in sys.stdin:
    line = line.rstrip()
    (op1, op, op2, _, out) = line.split()
    dot.node(out, out + " " + op)
    dot.edge(op1, out)
    dot.edge(op2, out)

dot.render()
