#include "hash.h"
#include <stdio.h>

int main(void)
{
    uint32_t old_salary = 0;
    hashRecord *found = NULL;

    insert_record("Alice", 50000, 100);
    insert_record("Charlie", 70000, 300);
    insert_record("Bob", 60000, 200);

    printf("After inserts:\n");
    print_table(stdout);

    found = search_record("Bob", 200);
    if (found != NULL)
    {
        printf("Found: %u,%s,%u\n", found->hash, found->name, found->salary);
    }

    update_record("Bob", 200, 65000, &old_salary);
		printf("Old Bob salary: %u\n", old_salary);

    delete_record("Alice", 100);

    printf("After update/delete:\n");
		print_table(stdout);

	if (insert_record("Bob", 99999, 200) == 0)
		printf("Duplicate rejected\n");

	

	if (delete_record("Nobody", 999) == 0)
		printf("Missing delete handled\n");



	if (update_record("Nobody", 999, 12345, &old_salary) == 0)
		printf("Missing update handled\n");

	free_table();

	insert_record("Zed", 1000, 300);
	insert_record("Ann", 1000, 100);
	insert_record("Mike", 1000, 200);

	print_table(stdout);
	
		


    free_table();
    return 0;
}


