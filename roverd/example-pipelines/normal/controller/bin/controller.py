import time

print("Starting Mock Python Controller", flush=True)

time.sleep(1)

counter = 0
while True:
    print(f"compute... {counter}", flush=True)
    time.sleep(1)
    counter += 1