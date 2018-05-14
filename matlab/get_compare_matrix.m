function [res] = get_compare_matrix(dim1, dim2, folder )


csf = 'cv_fits' %compare_stat_folder
stat = 'max';


if ~exist('folder','var')
   d = dir('./*');
   folder = '';
else
   d = strcat(folder,'/*');
end


[ss, cs] =  get_folder_conditions(dim1, dim2, folder )

res = zeros(length(ss), length(cs))



for f = d'
    fname = f.name
    n1 = fname(1);
    if n1 == '.'
        continue
    end
    
    fname = strcat(f.name, '/', csf, '/', stat, '.txt');
    
    [av, sd] = plot_distr(fname)
    
    [s, c] = folder_name_to_nums(f.name, dim1, dim2);
    si = find(ss==s);
    ci = find(cs==c);
    res(si,ci) = av;
end
ss 
cs
res