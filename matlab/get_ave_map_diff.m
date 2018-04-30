function M = get_ave_map_diff(root_dir)
% given the name of file (such as 'iter0-fold0.txt') and root_dir containing
% the folders with the maps (such as 's0_c0'
strcat(root_dir, '/test_cv_maps/','*.txt')
test_folder = dir(strcat(root_dir, '/test_fit_maps/','*.txt'))
count = 0;

for file = test_folder'
    the_file = file.name %gets stored as ans
    if count == 0
        M = get_test_cv_map_diff(the_file, root_dir);
    else
        M = M + get_test_cv_map_diff(the_file, root_dir);
    end
    count = count + 1;
end
M = M/count;

end