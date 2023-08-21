#include "foo.h"

#include <stdio.h>

void show(const unsigned char* data, size_t len)
{
	for (size_t i = 0; i < len; i++)
	{
		printf("%02x ", data[i]);
	}
}
