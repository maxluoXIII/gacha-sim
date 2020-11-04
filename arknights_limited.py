import random

results = {k: 0 for k in range(1, 301)}
num_simulations = 1000000
num_six_star_rate_ups = 2
num_targeted_rate_ups = 1
target_ratio = num_targeted_rate_ups / num_six_star_rate_ups
for sim_num in range(num_simulations):
    rolls_since_last_six_star = 0
    got_targeted_op = False
    for roll_num in range(1, 301):
        six_star_rate = 2 + max((rolls_since_last_six_star - 50) * 2, 0)
        if random.randint(1, 100) <= six_star_rate: # chance for six star
            if random.random() < target_ratio * 0.7: # odds of getting targeted operator
                results[roll_num] += 1
                got_targeted_op = True
                break
            rolls_since_last_six_star = 0
        else:
            rolls_since_last_six_star += 1
    if not got_targeted_op:
        results[300] += 1
# print(results)
sorted_results = {k: v/num_simulations for k, v in sorted(results.items(), key=lambda item: item[1], reverse=True)}
print(sorted_results)

saved_rolls = 91
cum_prob = 0
for i in range(1, saved_rolls+1):
    cum_prob += results[i] / num_simulations
print(cum_prob)
