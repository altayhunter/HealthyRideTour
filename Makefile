TARGET = main
CC = $(shell if command -v clang++ >/dev/null 2>&1; then echo "clang++"; else echo "g++"; fi)
SRC = $(wildcard *.cpp)
OBJ = $(patsubst %.cpp, %.o, $(SRC))
CFLAGS = -O3 -std=c++11 -Wall
DFLAGS = -O0 -std=c++11 -Wall -Wextra -g -fsanitize=integer -fsanitize=address -DDEBUG

.PHONY: debug run edit clean

$(TARGET).exe: $(OBJ)
	$(CC) $(CFLAGS) $(OBJ) -o $(TARGET).exe

$(OBJ): %.o : %.cpp
	$(CC) $(CFLAGS) -c $< -o $@

debug: $(SRC)
	$(CC) $(DFLAGS) $(SRC) -o $(TARGET).exe

run: $(TARGET).exe
	./$(TARGET).exe

edit:
	vim $(TARGET).cpp

clean:
	-rm -f *.stackdump *.exe *.o .*.swp *.gch
