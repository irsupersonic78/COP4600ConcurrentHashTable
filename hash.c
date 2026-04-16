#include "hash.h"

#include <stdlib.h>
#include <string.h>

static hashRecord *head = NULL;

static hashRecord *create_node(const char *name, uint32_t salary, uint32_t hash)
{
    hashRecord *node = malloc(sizeof(hashRecord));
    if (node == NULL)
    {
        return NULL;
    }

    node->hash = hash;
    strncpy(node->name, name, sizeof(node->name) - 1);
    node->name[sizeof(node->name) - 1] = '\0';
    node->salary = salary;
    node->next = NULL;

    return node;
}

hashRecord *search_record(const char *name, uint32_t hash)
{
    hashRecord *curr = head;

    while (curr != NULL)
    {
        if (curr->hash == hash && strcmp(curr->name, name) == 0)
        {
            return curr;
        }
        curr = curr->next;
    }

    return NULL;
}

int insert_record(const char *name, uint32_t salary, uint32_t hash)
{
    hashRecord *existing = search_record(name, hash);
    if (existing != NULL)
    {
        return 0;
    }

    hashRecord *node = create_node(name, salary, hash);
    if (node == NULL)
    {
        return -1;
    }

    if (head == NULL || hash < head->hash)
    {
        node->next = head;
        head = node;
        return 1;
    }

    hashRecord *prev = head;
    hashRecord *curr = head->next;

    while (curr != NULL && curr->hash < hash)
    {
        prev = curr;
        curr = curr->next;
    }

    node->next = curr;
    prev->next = node;

    return 1;
}

int update_record(const char *name, uint32_t hash, uint32_t new_salary, uint32_t *old_salary)
{
    hashRecord *record = search_record(name, hash);
    if (record == NULL)
    {
        return 0;
    }

    if (old_salary != NULL)
    {
        *old_salary = record->salary;
    }

    record->salary = new_salary;
    return 1;
}

int delete_record(const char *name, uint32_t hash)
{
    hashRecord *curr = head;
    hashRecord *prev = NULL;

    while (curr != NULL)
    {
        if (curr->hash == hash && strcmp(curr->name, name) == 0)
        {
            if (prev == NULL)
            {
                head = curr->next;
            }
            else
            {
                prev->next = curr->next;
            }

            free(curr);
            return 1;
        }

        prev = curr;
        curr = curr->next;
    }

    return 0;
}

void print_table(FILE *out)
{
    hashRecord *curr = head;

    while (curr != NULL)
    {
        fprintf(out, "%u,%s,%u\n", curr->hash, curr->name, curr->salary);
        curr = curr->next;
    }
}

void free_table(void)
{
    hashRecord *curr = head;

    while (curr != NULL)
    {
        hashRecord *next = curr->next;
        free(curr);
        curr = next;
    }

    head = NULL;
}
