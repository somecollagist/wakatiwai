SIZE_M=4

while read -r size; do
	VALUE=${size:1:${#size}-3}
	UNIT=${size:${#size}-2:1}

	case $UNIT in
		"M")
			SIZE_M=$((SIZE_M+$VALUE))
			;;
		
		"G")
			SIZE_M=$((SIZE_M+(1024*$VALUE)))
			;;

		*)
			echo "Unknown unit \"$UNIT\""
			exit 1
			;;
	esac
done < <(jq ".partitions[].size" $1)

echo $SIZE_M