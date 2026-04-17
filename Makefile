CC = rustc
OUT = chash
SRC = src/main.rs src/hash_table.rs src/logger.rs

all: $(OUT)

$(OUT): $(SRC)
	$(CC) src/main.rs --out-dir . -o $(OUT)

clean:
	rm -f $(OUT) hash.log

run: all
	./$(OUT)
