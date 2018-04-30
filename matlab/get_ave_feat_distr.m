function M = get_ave_feat_distr(root_dir)
% given the name of folder which contains eq sized matrixes, returns the
% average of them.
% f_name = 'eval6/test_fit_maps/'
root_dir = strcat(root_dir, '/feats/');
folder = dir(strcat(root_dir,'iter*.txt'));
count = 0;

for file = folder'
    the_file = file.name %gets stored as ans
    
    if count == 0
        M = dlmread( strcat(root_dir,the_file) );
    else
        M = M + dlmread( strcat(root_dir,the_file) );
    end
    count = count + 1;
end

M = M'/count;

end