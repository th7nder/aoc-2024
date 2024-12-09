disk = []
fid = 0

class File:
    def __init__(self):
        self.fid = -1
        self.size = 0
        self.empty = True
        self.next = None
        self.prev = None

root = None
curr = root

for (index, c) in enumerate(input()):
    size = int(c)
    if index % 2 == 0:
        disk += [fid] * size
        if root is None:
            root = File()
            root.empty = False
            root.size = size
            root.fid = fid
            curr = root
        else:
            prev = curr
            curr.next = File()
            curr = curr.next
            curr.empty = False
            curr.size = size
            curr.fid = fid
            curr.prev = prev
        fid += 1
    else:
        prev = curr
        disk += [-1] * size
        curr.next = File()
        curr = curr.next
        curr.empty = True
        curr.size = size
        curr.prev = prev

head = root
tail = curr

while tail != head and tail != None:
    while tail.empty and tail != head:
        tail = tail.prev
    if tail == head:
        break

    file = tail
    empty = head
    while not empty.empty or empty.size < file.size:
        if empty == file:
            break
        empty = empty.next

    if empty != file:
        # print("found space ", empty.size, " for file ", file.fid, " of size: ", file.size)
        empty.fid = file.fid
        empty.empty = False
        file.fid = -1
        file.empty = True
        
        if empty.size > file.size:
            new_empty = File()
            new_empty.size = empty.size - file.size
            new_empty.empty = True

            new_empty.prev = empty
            new_empty.next = empty.next
            new_empty.next.prev = new_empty
            empty.next = new_empty

        empty.size = file.size

    tail = tail.prev


checksum = 0

pos = 0
curr = head
while curr:
    if not curr.empty:
        # print(f"{curr.fid}" * curr.size, end='')
        for i in range(pos, pos + curr.size):
            checksum += curr.fid * i

    pos += curr.size
    curr = curr.next
# print()

print(checksum)