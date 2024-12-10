use itertools::Itertools;

const EXAMPLE: &str = "2333133121414131402";

fn parse_disk_map(input: &str) -> Vec<(u32, u32, u8)> {
    let input = input.trim();
    assert!(input.chars().all(|c| c.is_ascii_digit()));
    let counts = input.as_bytes().iter().copied().map(|b| b - b'0');

    let mut block_groups = Vec::with_capacity(
        // NOTE: Only a best guess, certainly not right. Hopefully avoids a couple of resizes.
        input.len(),
    );

    let mut file_true_empty_false = true;
    let mut block_idx = 0u32;
    let mut file_id = 0u32;
    for count in counts {
        if file_true_empty_false {
            block_groups.push((block_idx, file_id, count));
            file_id += 1;
        }
        block_idx += u32::from(count);
        file_true_empty_false = !file_true_empty_false;
    }

    block_groups
}

fn p1_compact_and_compute_checksum(input: &str) -> u64 {
    let mut block_groups = parse_disk_map(input);

    let mut checksum = 0u64;
    let mut acc_checksum = |pos, value| {
        checksum = checksum
            .checked_add(u64::from(pos) * u64::from(value))
            .unwrap();
    };
    let mut free_block_search_start_idx = 0u32;
    let mut next_block_group_idx = 0;
    while let Some(&(group_block_idx, _group_file_id, _group_count)) =
        block_groups.get(next_block_group_idx)
    {
        if group_block_idx != free_block_search_start_idx {
            let mut num_unallocated_discovered =
                u8::try_from(group_block_idx - free_block_search_start_idx).unwrap();
            while num_unallocated_discovered != 0 {
                let (_idx, rear_file_id, ref mut rear_group_count) =
                    block_groups.last_mut().unwrap();
                let num_to_reallocate_from_group =
                    num_unallocated_discovered.min(*rear_group_count);

                let new_free_block_search_start_idx =
                    free_block_search_start_idx + u32::from(num_to_reallocate_from_group);
                for block_idx in free_block_search_start_idx..(new_free_block_search_start_idx) {
                    acc_checksum(block_idx, *rear_file_id)
                }
                free_block_search_start_idx = new_free_block_search_start_idx;

                if num_to_reallocate_from_group == *rear_group_count {
                    block_groups.pop();
                } else {
                    *rear_group_count -= num_to_reallocate_from_group;
                }
                num_unallocated_discovered -= num_to_reallocate_from_group;
            }
        }
        assert_eq!(group_block_idx, free_block_search_start_idx);

        if let Some(&(group_block_idx, group_file_id, group_count)) =
            block_groups.get(next_block_group_idx)
        {
            let new_free_block_search_start_idx = group_block_idx + u32::from(group_count);
            for block_idx in group_block_idx..new_free_block_search_start_idx {
                acc_checksum(block_idx, group_file_id);
            }
            free_block_search_start_idx = new_free_block_search_start_idx;
        }

        next_block_group_idx += 1;
    }

    checksum
}

#[test]
fn p1_example() {
    assert_eq!(p1_compact_and_compute_checksum(EXAMPLE), 1928);
}

const INPUT: &str = include_str!("./d9.txt");

#[test]
fn p1() {
    assert_eq!(p1_compact_and_compute_checksum(INPUT), 6200294120911);
}

fn p2_soft_compact_and_compute_checksum(input: &str) -> u64 {
    let mut block_groups = parse_disk_map(input);

    let mut checksum = 0u64;
    let mut acc_checksum = |pos, value| {
        checksum = checksum
            .checked_add(u64::from(pos) * u64::from(value))
            .unwrap();
    };

    let mut maybe_move_block_group_idx = block_groups.len().checked_sub(1).unwrap();
    while maybe_move_block_group_idx != 0 {
        let (_maybe_move_group_block_idx, maybe_move_group_file_id, maybe_move_group_count) =
            block_groups[maybe_move_block_group_idx];

        if let Some((new_block_group_idx, new_block_idx)) = block_groups
            [..maybe_move_block_group_idx]
            .iter()
            .copied()
            .enumerate()
            .tuple_windows()
            .find_map(
                |(
                    (_group_1_idx, (group_1_start_idx, _file_id, group_1_count)),
                    (group_2_idx, (group_2_start_idx, ..)),
                )| {
                    let this_group_end_block_idx = group_1_start_idx + u32::from(group_1_count);
                    let fits = u32::from(maybe_move_group_count)
                        <= (group_2_start_idx - this_group_end_block_idx);
                    fits.then_some((group_2_idx, this_group_end_block_idx))
                },
            )
        {
            block_groups.remove(maybe_move_block_group_idx);
            block_groups.insert(
                new_block_group_idx,
                (
                    new_block_idx,
                    maybe_move_group_file_id,
                    maybe_move_group_count,
                ),
            );
        }

        maybe_move_block_group_idx -= 1;
    }

    for &(group_block_idx, group_file_id, group_count) in &block_groups {
        for block_idx in group_block_idx..group_block_idx + u32::from(group_count) {
            acc_checksum(block_idx, group_file_id);
        }
    }

    checksum
}

#[test]
fn p2_example() {
    assert_eq!(p2_soft_compact_and_compute_checksum(EXAMPLE), 2858);
}

#[test]
fn p2() {
    assert_eq!(p2_soft_compact_and_compute_checksum(INPUT), 2858);
}
