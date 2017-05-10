import os
import random

dec = "haff.dec"
enc = "haff.enc"
test = "test"

def some_test():
    some_list = [chr(i) for i in xrange(32, 127)]
    for count in xrange(1000):
        test_string = ''
        #I have protection against zero-length strings, so there is no need in generating of empty files
        for _ in xrange(random.randint(2,100)):
            test_string += random.choice(some_list)
        #dec_file = open(dec)
        #enc_file = open(enc)
        test_file = open(test, 'w')
        test_file.write(test_string)
        test_file.close()
        
        os.system("cargo run -- -f %s" % (test,))
        print "Test string for this round - '%s'" % (test_string,)
        dec_file = open(dec)
        test_file = open(test)
        a = dec_file.read()
        b = test_file.read()
        try:
            assert a == b
        except AssertionError:
            print "Here is a - %s\nHere is b - %s" % (a, b)
            return
        test_file.close()
        dec_file.close()
        print "Round", count
        os.unlink(dec)
        os.unlink(enc)
        os.unlink(test)
        

if __name__ == "__main__":
    try:
        os.unlink(dec)
        os.unlink(enc)
        os.unlink(test)
    except OSError:
        print "Well here is no initial files already"

    some_test()

