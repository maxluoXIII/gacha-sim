import random
import matplotlib.pyplot as plt
import argparse

def run_simulation(num_six_star_rate_ups, num_targeted_rate_ups):
    pass

def get_stats(num_simulations):
    results = {k: 0 for k in range(1, 301)}
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
    return results

def print_graphs(results, num_simulations, saved_rolls):
    sorted_results = {k: v/num_simulations for k, v in sorted(results.items(), key=lambda item: item[1], reverse=True)}
    print(sorted_results)

    plt.figure(1)
    plt.subplot(211)
    plt.bar(sorted_results.keys(), sorted_results.values(), 1.0)
    plt.title("Probability Distribution")
    plt.xlabel("Number of rolls")
    plt.ylabel("Probability")

    cum_probs = [0] * 301 # valid indices [0, 300]
    for i in range(1, 300+1):
        cum_probs[i] = cum_probs[i-1] + results[i] / num_simulations
    print(cum_probs[saved_rolls])

    plt.subplot(212)
    plt.bar(range(0, 301), cum_probs, width=1.0)
    plt.title("Cumulative Probability Distribution")
    plt.xlabel("Number of rolls")
    plt.ylabel("Cumulative probability")
    plt.axhline(cum_probs[saved_rolls], color="red")
    plt.axvline(saved_rolls, color="red")
    plt.annotate(f"{cum_probs[saved_rolls]:.2}", xy=(saved_rolls, cum_probs[saved_rolls]), xytext=(saved_rolls - 25, cum_probs[saved_rolls] + 0.1))
    plt.tight_layout()
    plt.savefig("arknights_limited.png")
    plt.show()

def get_rolls(orundum, single_tickets, ten_tickets):
    return orundum // 600 + single_tickets + ten_tickets * 10

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run simulations for Arknight limited banner")
    roll_group = parser.add_mutually_exclusive_group(required=True)
    roll_group.add_argument("--rolls", nargs=1, help="Number of rolls")
    roll_group.add_argument("--resources", nargs=3, help="Get number of rolls from <orundum>, <single-roll tickets>, and <ten-roll tickets>")


    args = parser.parse_args()
    saved_rolls = 0
    if args.rolls:
        saved_rolls = int(args.rolls[0])
    else:
        saved_rolls = get_rolls(*map(int, args.resources))

    num_simulations = 10000
    results = get_stats(num_simulations)
    print_graphs(results, num_simulations, saved_rolls)