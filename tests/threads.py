import threading
def client1():
   print("client 1 here")
def client2():
   print("client 2 here")
ts = [threading.Thread(target=f) for f in [client1, client2]]
for t in ts:
   t.start()
for t in ts:
   t.join()
