#!/bin/bash

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit 1
fi

hosts="$@"
filename=/etc/hosts

START_LINE="# Added by $CLUSTER_NAME"
END_LINE="# End of section"

block_start=$(grep -m 1 -n "$START_LINE" $filename | cut -f1 -d:)

if [[ "$block_start" == "" ]]; then
    echo "First time adding hosts: creating managed block in /etc/hosts file..."

    echo "" >> $filename
    echo "$START_LINE" >> $filename

    for host in $hosts; do
        echo "127.0.0.1      $host.ponglehub.co.uk" >> $filename
    done

    echo "$END_LINE" >> $filename

    echo "Done!"
else
    let end_of_first_section=$block_start-1
    length=$(tail -n +$block_start $filename | grep -m 1 -i -n "# End of section" | cut -f1 -d:)
    let start_of_second_section=$length+$block_start

    start_of_file=$(head -n $end_of_first_section $filename)
    end_of_file=$(tail -n +$start_of_second_section $filename)

    echo "$start_of_file" > $filename
    echo "" >> $filename
    echo "$START_LINE" >> $filename

    for host in $hosts; do
        echo "127.0.0.1      $host.ponglehub.co.uk" >> $filename
    done

    echo "$END_LINE" >> $filename
    echo "$end_of_file" >> $filename
    echo "Updated /etc/hosts file"
fi