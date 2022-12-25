// Zadanie: Mapa nieba
// Autor: Radoslaw Dabkowski
// Nr indeksu: 325683

/*************************************************************************
*
*  Weighted quick union implementation of union-find algotithm
*  with path compression
*
*  A reference implementation
*
*
*  Author: Jacek Marciniak
*  Date: 2015-11-07
*  
*************************************************************************/

#include <iostream>
#include <vector>
#include <string.h>

using namespace std;


// data
int cc;			// number of components
int N;			// number of elements
vector<int> id; // id[i] = parent of i
vector<int> sz; // sz[i] = number of elements in subtree rooted at i

// initializes data structures
void init(int size)
{
	cc = size;
	N = size;

	id.resize(size);
	sz.resize(size);

	for (int i = 0; i < N; i++)
	{
		id[i] = i;
		sz[i] = 1;
	}
}

// gets component's identifier of the element
int find(int p)
{
	int r = p;

	while (r != id[r]) // find
	{
		r = id[r];
	}

	while (p != r) // path compression
	{
		int n = id[p];
		id[p] = r;
		p = n;
	}

	return r;
}

// check if 2 elements belongs to the same component
bool connected(int p, int q)
{
	return find(p) == find(q);
}

// connect components
void make_union(int p, int q)
{
	int pid = find(p);
	int qid = find(q);
	
	if (pid == qid) return;

	// make smaller root point to larger one
	if (sz[qid] < sz[pid])
	{
		id[qid] = pid;
		sz[pid] += sz[qid];
	}
	else
	{
		id[pid] = qid;
		sz[qid] += sz[pid];
	}
	
	cc--;
}


int main() {
	// initialize data structures
	int n;
	cin >> n;

	init(n*n);
	vector<int> map(n*n);

	// read the map
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < n; j++)
		{
			cin >> map[i*n + j];
		}
	}

	// find all constellations
	for (int i = 0; i < n; i++) {
		for (int j = 0; j < n; j++) {
			// if there is a star
			if (map[i*n + j] == 1) {
				// check if there is a star to the right
				if (j < n - 1 && map[i*n + j + 1] == 1) {
					make_union(i*n + j, i*n + j + 1);
				}

				// check if there is a star below
				if (i < n - 1 && map[(i + 1)*n + j] == 1) {
					make_union(i*n + j, (i + 1)*n + j);
				}
			}
		}
	}

	// find the biggest constellation and count all constellations
	int max_size = 0;
	int constellations = 0;
	for (int i = 0; i < n*n; i++) {
		if (map[i] == 1) {
			// count constellations
			if (id[i] == i && map[i] == 1) {
				constellations++;
			}
			// find the biggest one
			if (sz[i] > max_size) {
				max_size = sz[i];
			}
		}
	}

	cout << constellations << " " << max_size;
}