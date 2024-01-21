## Optimal Reservation Calculation for Vehicles

To calculate the optimal reservations for vehicles over a day, we first define the following elements:

- **v0**: A vector containing the registry of cars.
- **v1**: A vector containing the registry of truck1.
- **v2**: A vector containing the registry of truck2.

These vectors are sorted in descending order based on the departure time.

### Algorithm Application

Slots Allocation

- Total Slots: 10 (5 for algorithm allocation, 5 reserved for walk-ins).

#### Time Definitions

- **T0**: Time of the slot with the earliest starting point or an increment thereof.
- **T1**: Departure time of the last truck1 in v1.  
  Define $\Delta T1 = T1 - T0$.
- **T2**: Departure time of the last truck2 in v2.  
  Define $\Delta T2 = T2 - T0$.
- **Tv1**: Departure time of the last car in v0.  
  Define $\Delta Tv1 = Tv1 - T0$.
- **Tv2**: Departure time of the second-to-last car in v0.  
  Define $\Delta Tv2 = Tv2 - T0$.

#### Pricing Per Minute

- **Car**: $\$5/\text{min}$ (Calculated as $150/30$).
- **Truck1**: $\$4.166/\text{min}$ (Calculated as $250/60$).
- **Truck2**: $\$6/\text{min}$ (Calculated as $750/120$).

Calculations:
- $(750 - 4 \times 150) / 5 = 30$ minutes. Hence, Truck2 remains more cost-effective compared to a car as it requires 30 more minutes to be equivalent in cost.
- $(250 - 2 \times 150) / 4.1666 = 12$ minutes. Hence, Truck1 can be more cost-effective if $\Delta Tv1 > 72$ minutes.
- $(750 - 2 \times 250) / 4.1666 = 60$ minutes. Hence, Truck1 can be more cost-effective if $\Delta T2 > 180$ minutes.

#### Algorithm Logic

Start with `T0 = min(slot)` and iterate until `T0 < 19`:

1. `If` $\Delta T2 < 30$: Select the last Truck2.
2. `Else if` $\Delta T2 \leq 180$:
    - If $T2 - Tv1 >= 30$: Select the last car.
    - `Else`: Select Truck2.
3. `Else`:
    - `If` $Tv1 - T1 > 12$ and $\Delta T2 < 120$: Select the last Truck1.
    - `Else if` $\Delta T1 + 30 < \Delta Tv2$ and $\Delta T2 < 120$: Select the last Truck1.
    - `Else`:
        - `If` $\Delta Tv1 + 30 < 180$: Select the last car.
        - `Else`: `T0 += 30`.
4. `If` an element is selected, update `T0 = min(slot)`.

### Complexity Analysis

The algorithm was determined based on the above elements and offers a complexity of `O(n)` for finding the optimal reservations and `O(nlog(n))` for sorting the vectors.
