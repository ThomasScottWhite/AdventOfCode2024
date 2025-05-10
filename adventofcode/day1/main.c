#include <stdio.h>
#include <stdlib.h>

#include "uthash.h"

typedef struct
{
    int key;
    int count;
    UT_hash_handle hh;
} HashEntry;

// This is used by QTSort
int compare_ints(const void *a, const void *b)
{
    return (*(int *)a - *(int *)b);
}

int count_lines(const char *filename)
{
    FILE *file = fopen(filename, "r");
    if (file == NULL)
    {
        perror("Failed to open file");
        return 1;
    }

    // This counts the number of lines inside of the input file
    int count = 0;
    char c;
    while ((c = fgetc(file)) != EOF)
    {
        if (c == '\n')
        {
            count++;
        }
    }
    return count;
}

int main()
{
    const char *filename = "input.txt";
    int count = count_lines(filename);

    // This creates two lists of the length of the input file
    int list1[count], list2[count];

    FILE *file = fopen(filename, "r");
    if (file == NULL)
    {
        perror("Failed to open file");
        return 1;
    }

    for (int i = 0; i < count; i++)
    {
        fscanf(file, "%d %d", &list1[i], &list2[i]);
    }

    qsort(list1, count, sizeof(int), compare_ints);
    qsort(list2, count, sizeof(int), compare_ints);

    int difference_tally = 0;
    for (int i = 0; i < count; i++)
    {
        difference_tally = difference_tally + abs(list1[i] - list2[i]);
    }

    printf("%d\n", difference_tally);

    HashEntry *map = NULL;

    for (int i = 0; i < count; i++)
    {
        HashEntry *entry;
        int key = list2[i];

        HASH_FIND_INT(map, &key, entry);
        if (entry == NULL)
        {
            entry = malloc(sizeof(HashEntry));
            entry->key = key;
            entry->count = 1;
            HASH_ADD_INT(map, key, entry);
        }
        else
        {
            entry->count++;
        }
    }

    int similuarlity_score = 0;
    for (int i = 0; i < count; i++)
    {
        HashEntry *entry;
        int key = list1[i];
        HASH_FIND_INT(map, &key, entry);
        int value = (entry != NULL) ? entry->count : 0;
        similuarlity_score = similuarlity_score + key * value;
    }
    printf("%d\n", similuarlity_score);

    return 0;
}