BEGIN {
	FS="\t";

	# Return a size in M (1024)
	# Add some to account for padded sectors, GPT, etc.
	SIZE_M=4
}

{
	value=substr($2, 0, length($2)-1);
	unit=substr($2, length($2), 1);

	if (unit == "G") {
		value *= 1024
	}

	SIZE_M += value
}

END {
	print SIZE_M
}