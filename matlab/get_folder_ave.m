function M = get_folder_ave(f_name)
% given the name of folder which contains eq sized matrixes, returns the
% average of them.
% f_name = 'eval6/test_fit_maps/'

folder = dir(strcat(f_name,'*.txt'));
count = 0;

for file = folder'
    the_file = file.name %gets stored as ans
    if count == 0
        M = dlmread( strcat(f_name,the_file) );
    else
        M = M + dlmread( strcat(f_name,the_file) );
    end
    count = count + 1;
end

M = M/count;

end
