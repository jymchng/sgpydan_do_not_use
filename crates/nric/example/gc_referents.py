import gc
from nric_do_not_use import SecretNRIC

   
if __name__ == '__main__':
    w = list()
    for i in ['S1234567D', 'S6364259B']:
        w.append(SecretNRIC(i, '.env.example', 'SECRET_KEY'))
        del i
    
    print(gc.get_referents(w))
    print('='*10)
    print(gc.get_referrers(w))
    gc.collect()
    print(gc.get_referents(w))
    print('='*10)
    print(gc.get_referrers(w))