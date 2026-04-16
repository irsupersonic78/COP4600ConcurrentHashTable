#ifndef HASH_H
#define HASH_H

#include <stdint.h>
#include <stdio.h>

typedef struct hash_struct
{
    uint32_t hash;
    char name[50];
    uint32_t salary;
    struct hash_struct *next;
} hashRecord;

int insert_record(const char *name, uint32_t salary, uint32_t hash);
int delete_record(const char *name, uint32_t hash);
hashRecord *search_record(const char *name, uint32_t hash);
int update_record(const char *name, uint32_t hash, uint32_t new_salary, uint32_t *old_salary);
void print_table(FILE *out);
void free_table(void);

#endif
