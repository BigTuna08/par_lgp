function M = get_test_cv_map_diff(f_name, root_dir)
% given the name of file (such as 'iter0-fold0.txt') and root_dir containing
% the folders with the maps (such as 's0_c0'

 M_test = dlmread( strcat(root_dir, '/test_fit_maps/', f_name) );
 M_cv = dlmread( strcat(root_dir, '/cv_fit_maps/', f_name) );
 M = M_test - M_cv;