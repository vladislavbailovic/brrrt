int main() {
	int arr[5] = {
		12,13,1,6,17,
	};
	int val = 0;
	int limit = sizeof(arr) / sizeof(arr[0]);
	for (int i = 0; i < limit; i++) {
		val += arr[i];
	}
	return val;
}
