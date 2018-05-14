function [first_dim, second_dim] = get_folder_conditions(dim1, dim2, folder )
% find how many s and c's there are, to create matrix for comparing

if ~exist('folder','var')
   d = dir('./*');
else
   d = dir(strcat(folder,'./*'));
end


first_dim = [];
second_dim = [];
d
for f = d'
    fname = f.name
    n1 = fname(1);
    if n1 == '.'
        continue
    end
    
    [a1 a2] = folder_name_to_nums(f.name, dim1, dim2);
    first_dim = [first_dim a1];
    second_dim = [second_dim a2];
end
first_dim = unique(first_dim);
second_dim = unique(second_dim);
