BEGIN {
	FS="\t";

	# Keeps track of how many partitions we're dealing with
	CURRENT_PARTITION=0;

	# Create GPT
	print "g";
}

{
	# Create new partition
	print "n";
	CURRENT_PARTITION+=1;
	# Use next partition number
	print "";
	# Use next available sector
	print "";
	# Set size
	print "+"$2;
	# Specify type
	print "t";
	# If more than one partition exists, specify:
	if (CURRENT_PARTITION > 1) {
		print CURRENT_PARTITION;
	}
	# Set type
	if ($1 == "BOOT") {
		print "1";
	}
	else {
		print $1
	}
}

END {
	# Write changes
	print "w";
}