seq = input("Please enter your input: ")
total = 0
for i, val in enumerate(seq):
	n = (i + 1) % len(seq)
	if val==seq[n]:
		total += val
print total
