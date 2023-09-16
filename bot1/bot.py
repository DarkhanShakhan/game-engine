#!/usr/bin/env python3


import sys
import math

# Функция для поиска ближайшего нейтрального города
def find_strongest_city(cities, my_owner='p1'):
    my_cities = {name: city for name, city in cities.items() if city['owner'] == my_owner}
    max_units = -1
    strongest_city = None
    
    for city_name, city in my_cities.items():
        if city['units'] > max_units:
            max_units = city['units']
            strongest_city = city_name
    
    return strongest_city

def find_nearest_neutral_city(cities, my_strongest_city):
    nearest_city = None
    nearest_distance = float('inf')
    
    if my_strongest_city is None:
        return None, float('inf')
    
    my_city = cities[my_strongest_city]
    
    for city_name, city in cities.items():
        if city['owner'] != 'Neutral':
            continue
        
        distance = math.sqrt((city['x'] - my_city['x']) ** 2 + (city['y'] - my_city['y']) ** 2)
        
        if distance < nearest_distance:
            nearest_distance = distance
            nearest_city = city
    
    return nearest_city



def count_my_cities(cities, my_owner='p1'):
    my_cities_count = 0
    for city in cities.values():
        if city['owner'] == my_owner:
            my_cities_count += 1
    return my_cities_count

def find_special_cities(cities, my_owner='p1'):
    my_cities = {name: city for name, city in cities.items() if city['owner'] == my_owner}
    
    closest_city = None
    closest_distance = float('inf')
    
    second_closest_city = None
    
    strongest_city = None
    max_units = -1
    
    for my_city_name, my_city in my_cities.items():
        my_units = my_city['units']
        
        if my_units > max_units:
            max_units = my_units
            strongest_city = my_city  # сохраняем сам город
            
        for city_name, city in cities.items():
            if city['owner'] != 'Neutral':
                continue
            
            distance = math.sqrt((city['x'] - my_city['x']) ** 2 + (city['y'] - my_city['y']) ** 2)
            
            if distance < closest_distance:
                closest_distance = distance
                closest_city = my_city  # сохраняем сам город
                
    if len(my_cities) >= 2 and closest_city:
        second_closest_distance = float('inf')
        for my_city_name, my_city in my_cities.items():
            if my_city == closest_city:
                continue
            distance = math.sqrt((closest_city['x'] - my_city['x']) ** 2 + (closest_city['y'] - my_city['y']) ** 2)
            
            if distance < second_closest_distance:
                second_closest_distance = distance
                second_closest_city = my_city  # сохраняем сам город
                
    return closest_city, strongest_city, second_closest_city



def count_cities_by_owner(cities, owner):
    count = 0
    for city in cities.values():
        if city['owner'] == owner:
            count += 1
    return count

# В основном цикле:




while True:

    # Чтение N, P, T
    N, P, T = input().split()
    T = int(T)  # преобразование T в целое число, поскольку оно всегда будет числом
    N = int(N)  # преобразование N в целое число, поскольку оно всегда будет числом
    # Чтение данных о городах
    cities = {}
    for _ in range(N):
        owner, units, name, x, y, param1, param2 = input().split()
        units, x, y, param1, param2 = map(int, [units, x, y, param1, param2])
        cities[name] = {'owner': owner, 'units': units, 'x': x, 'y': y, 'param1': param1, 'param2': param2}

    # Чтение M
    M = int(input())

    # Чтение данных о передвижениях
    moves = []
    for _ in range(M):
        from_city, to_city, from_owner, to_owner, ticks_left, units_moving = input().split()
        ticks_left, units_moving = map(int, [ticks_left, units_moving])
        moves.append({'from_city': from_city, 'to_city': to_city, 'from_owner': from_owner, 'to_owner': to_owner, 'ticks_left': ticks_left, 'units_moving': units_moving})

    nearest_city = find_nearest_neutral_city(cities, find_strongest_city(cities, my_owner='p1'))
    closest_city, strongest_city, second_closest_city = find_special_cities(cities, my_owner='p1')
    my_city = cities[find_strongest_city(cities, my_owner='p1')]
    
    my_city_count = count_cities_by_owner(cities, 'p1')
    opponent_city_count = count_cities_by_owner(cities, 'Neutral')
    # print(f"Number of my cities: {my_city_count}", file=sys.stderr, flush=True)
    # print(f"Number of opponent's cities: {opponent_city_count}", file=sys.stderr, flush=True)
    if my_city_count>2:
        print(str(second_closest_city['x'])+" "+str(second_closest_city['y'])+" "+str(closest_city['x'])+" "+str(closest_city['y']),file=sys.stderr, flush=True)


        # use file=sys.stderr to print for debugging
    if nearest_city is not None:
        if 2<=count_my_cities(cities, my_owner='p1'):
            print(my_city['units'], type(my_city), file=sys.stderr, flush=True)
    else:
        if 2<=count_my_cities(cities, my_owner='p1'):
            print(my_city, type(my_city), file=sys.stderr, flush=True)
    # this will choose one of random actions
    if nearest_city :
        if my_city['units']>=nearest_city['units']:
            print(str(my_city['x'])+" "+str(my_city['y'])+" "+str(nearest_city['x'])+" "+str(nearest_city['y']), flush=True)

        else:
            if opponent_city_count<my_city_count:
                print(str(strongest_city['x'])+" "+str(strongest_city['y'])+" " +str(nearest_city['x'])+" "+str(nearest_city['y']), flush=True )   
            else:
                if my_city_count>2:
                    print("helloworld", file=sys.stderr, flush=True)
                    print(str(second_closest_city['x'])+" "+str(second_closest_city['y'])+" "+str(closest_city['x'])+" "+str(closest_city['y']), flush=True)
                else:
                    print(str(closest_city['x'])+" "+str(closest_city['y'])+" "+str(strongest_city['x'])+" "+str(strongest_city['y']), flush=True)
                    print("", flush=True)
 
    else:
        if 2<=count_my_cities(cities, my_owner='p1'):
            print(str(second_closest_city['x'])+" "+str(second_closest_city['y'])+" " +str(nearest_city['x'])+" "+str(nearest_city['y']), flush=True)   
        else:
            print("", flush=True)